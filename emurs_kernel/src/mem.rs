use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    mem::size_of,
    ops::RangeInclusive,
};
use lock_api::{Mutex, RawMutex};
use tinyvec::ArrayVec;

/// FIXME: We need a way for memory tables to be reloading safely

/// The number of slabs the allocator can hold at a time before it panics
const SLAB_COUNT: usize = size_of::<usize>() * 2;

/// The global allocator for the operating system
#[cfg(feature = "embedded")]
#[global_allocator]
pub static mut EMURS_GLOBAL_MEMORY_ALLOCATOR: EmuRsAllocator<spin::mutex::Mutex<()>> =
    EmuRsAllocator::<spin::Mutex<()>>::new();

/// Type representing a memory range
/// This was used instead of [RangeInclusive] due to it not supporting [Copy] 
#[derive(Debug, Default, Copy, Clone)]
pub struct EmuRsMemoryRange {
    pub first: usize,
    pub last: usize,
}

impl EmuRsMemoryRange {
    pub fn new(first: usize, last: usize) -> Self {
        return Self { first, last };
    }

    pub fn range(&self) -> RangeInclusive<usize> {
        return self.first..=self.last;
    }
}

/// The permissions that a memory block has.
/// 
/// This will eventually merge with VFS permissions
#[derive(Debug, Default)]
pub struct EmuRsMemoryPermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// A brief description of the memory
/// 
/// Currently ignored
#[derive(Debug, Default)]
pub enum EmuRsMemoryKind {
    #[default]
    Reserved,
    Work,
    KernelStack,
}

/// A entry in the memory table used to determine where to allocate memory
#[derive(Debug, Default)]
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
pub struct EmuRsAllocator<MUTEX: RawMutex> {
    memory_table: Mutex<MUTEX, Option<EmuRsMemoryTable>>,
    slabs: Mutex<MUTEX, ArrayVec<[EmuRsAllocatorSlab; SLAB_COUNT]>>,
}

impl<MUTEX: RawMutex> EmuRsAllocator<MUTEX> {
    pub const fn new() -> Self {
        return Self {
            memory_table: Mutex::new(None),
            // This is a extremely messed up hack to make up for rust consts being really messed up
            slabs: Mutex::new(ArrayVec::from_array_empty(
                [EmuRsAllocatorSlab {
                    coverage: EmuRsMemoryRange { first: 0, last: 0 },
                }; SLAB_COUNT],
            )),
        };
    }

    /// Set the memory table. 
    /// 
    /// WARNING: It is completely and totally undefined to do this more than once, but I'm not locking it behind a OnceCell just yet
    pub unsafe fn set_memory_table(&mut self, table: EmuRsMemoryTable) {
        *self.memory_table.get_mut() = Some(table);
    }

    /// Get the ranges of free memory according to the memory and slab table
    /// This may seem slow and complicated but so far the compiler more or less wipes away every iterator
    pub fn get_free_memory_ranges(&self) -> ArrayVec<[EmuRsMemoryRange; 128]> {
        return self
            .memory_table
            .lock()
            .as_ref()
            .unwrap()
            .entries
            .iter()
            .flat_map(|entry| {
                // Gotta make sure this doesn't overflow
                let mut to_return = ArrayVec::<[EmuRsMemoryRange; SLAB_COUNT]>::new();

                // Go through every slab
                for slab in self.slabs.lock().iter() {
                    // Get free memory at the end
                    if entry.range.last > slab.coverage.last {
                        to_return.push(EmuRsMemoryRange::new(
                            self.slabs.lock()[0].coverage.last + 1,
                            entry.range.last,
                        ));
                    }

                    // Get free memory at the start
                    if entry.range.first < slab.coverage.first {
                        to_return.push(EmuRsMemoryRange::new(
                            entry.range.first,
                            self.slabs.lock()[0].coverage.first - 1,
                        ));
                    }
                }

                return to_return;
            })
            .collect();
    }

    /// Get a memory block that satifies the [Layout] passed in
    /// 
    /// FIXME: Currently ignores requested alignment.
    pub fn get_free_memory_block(&self, layout: Layout) -> EmuRsMemoryRange {
        return self
            .get_free_memory_ranges()
            .into_iter()
            .find(|block| {
                return block.range().count() >= layout.size();
            })
            .unwrap();
    }
}

unsafe impl<MUTEX: RawMutex> GlobalAlloc for EmuRsAllocator<MUTEX> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Make sure we have a memory table
        if self.memory_table.lock().is_none() {
            panic!();
        }

        let free_mem = self.get_free_memory_block(layout);

        self.slabs
            .lock()
            .push(EmuRsAllocatorSlab { coverage: free_mem });

        return free_mem.first as *mut u8;
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
