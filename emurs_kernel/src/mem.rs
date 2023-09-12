use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    mem::size_of,
    ops::RangeInclusive,
};
use lock_api::{Mutex, RawMutex};
use tinyvec::ArrayVec;

const SLAB_COUNT: usize = size_of::<usize>() * 2;

#[cfg(feature = "embedded")]
#[global_allocator]
pub static mut EMURS_GLOBAL_MEMORY_ALLOCATOR: EmuRsAllocator<spin::mutex::Mutex<()>> =
    EmuRsAllocator::<spin::Mutex<()>>::new();

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

#[derive(Debug, Default)]
pub struct EmuRsMemoryPermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

#[derive(Debug, Default)]
pub enum EmuRsMemoryKind {
    #[default]
    Reserved,
    Work,
    KernelStack,
}

#[derive(Debug, Default)]
pub struct EmuRsMemoryTableEntry {
    pub permissions: EmuRsMemoryPermission,
    pub range: EmuRsMemoryRange,
    pub kind: EmuRsMemoryKind,
}

#[derive(Debug, Default)]
pub struct EmuRsMemoryTable {
    pub entries: ArrayVec<[EmuRsMemoryTableEntry; 10]>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EmuRsAllocatorSlab {
    pub coverage: EmuRsMemoryRange,
}

pub struct EmuRsAllocator<MUTEX: RawMutex> {
    // For now this is a Oncecell until we figure out something better to do
    memory_table: Mutex<MUTEX, Option<EmuRsMemoryTable>>,
    slabs: Mutex<MUTEX, ArrayVec<[EmuRsAllocatorSlab; SLAB_COUNT]>>,
}

impl<MUTEX: RawMutex> EmuRsAllocator<MUTEX> {
    pub const fn new() -> Self {
        return Self {
            memory_table: Mutex::new(None),
            // This is a extremely fucked up hack to make up for rust consts being really fucked up
            slabs: Mutex::new(ArrayVec::from_array_empty(
                [EmuRsAllocatorSlab {
                    coverage: EmuRsMemoryRange { first: 0, last: 0 },
                }; SLAB_COUNT],
            )),
        };
    }

    pub unsafe fn set_memory_table(&mut self, table: EmuRsMemoryTable) {
        *self.memory_table.get_mut() = Some(table);
    }

    pub fn get_free_memory_ranges(&self) -> ArrayVec<[EmuRsMemoryRange; 128]> {
        return self
            .memory_table
            .lock()
            .as_ref()
            .unwrap()
            .entries
            .iter()
            .flat_map(|entry| {
                // Gotta make sure this doesn't overflow lmfao
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
