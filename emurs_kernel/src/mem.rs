use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    mem::size_of,
    ops::RangeInclusive,
};
use lock_api::{Mutex, RawMutex};
use tinyvec::{array_vec, ArrayVec};

/// FIXME: We need a way for memory tables to be reloading safely

/// The number of slabs the allocator can hold at a time before it panics
const SLAB_COUNT: usize = 128;

/// The global allocator for the operating system
#[global_allocator]
pub static mut EMURS_GLOBAL_MEMORY_ALLOCATOR: EmuRsAllocator = EmuRsAllocator::new();

fn align_address_upward(alignment: usize, addr: usize) -> usize {
    return (addr - (addr % alignment)) + alignment;
}

fn align_address_downward(alignment: usize, addr: usize) -> usize {
    return addr - addr % alignment;
}

/// Type representing a memory range
/// This was used instead of [RangeInclusive] due to it not supporting [Copy]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EmuRsMemoryRange {
    pub first: usize,
    pub last: usize,
}

impl EmuRsMemoryRange {
    pub const fn new(first: usize, last: usize) -> Self {
        return Self { first, last };
    }

    /// Checks if a range is inside or equal to this one
    pub fn contains_range(&self, range: EmuRsMemoryRange) -> bool {
        return range.first >= self.first && range.last <= self.last;
    }

    /// Checks if this range is overlapping
    pub fn overlaps_range(&self, range: EmuRsMemoryRange) -> bool {
        return range.first <= self.last && self.first <= range.last;
    }

    pub fn range(&self) -> RangeInclusive<usize> {
        return self.first..=self.last;
    }
}

/// The permissions that a memory block has.
///
/// This will eventually merge with VFS permissions
#[derive(Debug, Default, Clone, Copy)]
pub struct EmuRsMemoryPermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// A brief description of the memory
///
/// Currently ignored
#[derive(Debug, Default, Clone, Copy)]
pub enum EmuRsMemoryKind {
    #[default]
    Reserved,
    Work,
    KernelStack,
}

/// A entry in the memory table used to determine where to allocate memory
#[derive(Debug, Default, Clone, Copy)]
pub struct EmuRsMemoryTableEntry {
    pub permissions: EmuRsMemoryPermission,
    pub range: EmuRsMemoryRange,
    pub kind: EmuRsMemoryKind,
}

/// A memory table that must be passed in by the bootloader for allocation to occur
#[derive(Debug, Default)]
pub struct EmuRsMemoryTable {
    pub entries: ArrayVec<[EmuRsMemoryTableEntry; 10]>,
}

/// A entry in the internal allocation tracking table
#[derive(Debug, Default, Clone, Copy)]
struct EmuRsAllocatorSlab {
    pub coverage: EmuRsMemoryRange,
}

/// Implements a global allocator for the operating system
pub struct EmuRsAllocator {
    memory_table: Mutex<spin::Mutex<()>, EmuRsMemoryTable>,
    slabs: Mutex<spin::Mutex<()>, ArrayVec<[EmuRsAllocatorSlab; SLAB_COUNT]>>,
}

impl EmuRsAllocator {
    pub const fn new() -> Self {
        // This is a extremely messed up hack to make up for rust consts being really messed up
        let mut my_table = Self {
            memory_table: Mutex::new(EmuRsMemoryTable {
                entries: ArrayVec::from_array_empty(
                    [EmuRsMemoryTableEntry {
                        range: EmuRsMemoryRange::new(0, 0),
                        permissions: EmuRsMemoryPermission {
                            read: false,
                            write: false,
                            execute: false,
                        },
                        kind: EmuRsMemoryKind::Reserved,
                    }; 10],
                ),
            }),
            slabs: Mutex::new(ArrayVec::from_array_empty(
                [EmuRsAllocatorSlab {
                    coverage: EmuRsMemoryRange { first: 0, last: 0 },
                }; SLAB_COUNT],
            )),
        };

        return my_table;
    }

    /// Indicate where more memory might be
    pub fn add_memory_table_entries(&mut self, entries: &[EmuRsMemoryTableEntry]) {
        self.memory_table
            .get_mut()
            .entries
            .extend_from_slice(entries);
    }

    /// Get the ranges of free memory according to the memory and slab table
    /// This may seem slow and complicated but so far the compiler more or less wipes away every iterator
    pub fn get_free_memory_ranges(&self, alignment: usize) -> ArrayVec<[EmuRsMemoryRange; 128]> {
        return self
            .memory_table
            .lock()
            .entries
            .iter()
            .flat_map(|entry| {
                // Gotta make sure this doesn't overflow
                let mut to_return: ArrayVec<[EmuRsMemoryRange; SLAB_COUNT]> =
                    array_vec![entry.range];

                for slab in self.slabs.lock().iter() {
                    to_return = to_return
                        .into_iter()
                        .filter_map(|entry| {
                            // Its completely overlapped so remove it
                            if slab.coverage.contains_range(entry) {
                                return None;
                            }

                            if slab.coverage.overlaps_range(entry) {
                                let mut accepted = ArrayVec::<[EmuRsMemoryRange; 2]>::new();

                                // Get free memory at the end
                                if entry.range().contains(&slab.coverage.last) {
                                    // Trying to align
                                    let new_addr =
                                        align_address_upward(alignment, slab.coverage.last + 1);

                                    accepted.push(EmuRsMemoryRange::new(new_addr, entry.last));
                                }

                                // Get free memory at the start
                                if entry.range().contains(&slab.coverage.first) {
                                    // Trying to align
                                    let new_addr =
                                        align_address_downward(alignment, slab.coverage.first - 1);

                                    accepted.push(EmuRsMemoryRange::new(entry.first, new_addr));
                                }

                                return Some(accepted);
                            }

                            let mut tmp = ArrayVec::new();
                            tmp.push(entry);
                            return Some(tmp);
                        })
                        .flatten()
                        .collect();
                }

                return to_return;
            })
            .collect();
    }

    /// Get a memory block that satifies the [Layout] passed in
    pub fn get_free_memory_block(&self, layout: Layout) -> Option<EmuRsMemoryRange> {
        let result = self
            .get_free_memory_ranges(layout.align())
            .into_iter()
            .find(|block| {
                return block.range().count() >= layout.size();
            });

        if result.is_none() {
            return None;
        }

        return Some(EmuRsMemoryRange {
            first: result.unwrap().first,
            last: result.unwrap().first + layout.size() - 1,
        });
    }
}

unsafe impl GlobalAlloc for EmuRsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Make sure we have a memory table
        if self.memory_table.lock().entries.is_empty() {
            panic!();
        }

        let free_mem = self.get_free_memory_block(layout);

        self.slabs.lock().push(EmuRsAllocatorSlab {
            coverage: free_mem.unwrap(),
        });

        return free_mem.unwrap().first as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let num = self
            .slabs
            .lock()
            .iter()
            .enumerate()
            .find(|block| {
                return block.1.coverage.first == ptr as usize;
            })
            .unwrap()
            .0;

        self.slabs.lock().remove(num);
    }
}
