use crate::device::EmuRsDevice;
use crate::driver::EmuRsDriver;
use core::ptr::NonNull;

/// The disk implementation for filesystems to write and read
pub trait EmuRsDiskDriver: EmuRsDriver {
    /// TODO: Allow failure
    fn read(&self, buffer: &[u8], offset: usize);
    /// TODO: Allow failure
    fn write(&self, buffer: &mut [u8], offset: usize);
}

/// A dummy driver
pub struct EmuRsDummyDiskDriver;

impl EmuRsDriver for EmuRsDummyDiskDriver {
    fn name(&self) -> &str {
        return "Dummy Disk Driver";
    }

    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {}
}

impl EmuRsDiskDriver for EmuRsDummyDiskDriver {
    fn read(&self, buffer: &[u8], offset: usize) {}
    fn write(&self, buffer: &mut [u8], offset: usize) {}
}

/// A disk that just points somewhere in memory. Useful for the GBA save slot
pub struct EmuRsMemoryDisk {
    location: NonNull<[u8]>,
}

impl EmuRsDriver for EmuRsMemoryDisk {
    fn name(&self) -> &str {
        return "Memory Disk";
    }

    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {}
}

impl EmuRsMemoryDisk {
    /// TODO: Perhaps rethink using NonNull
    pub fn new(location: NonNull<[u8]>) -> Self {
        return Self { location };
    }
}

impl EmuRsDiskDriver for EmuRsMemoryDisk {
    fn read(&self, buffer: &[u8], offset: usize) {
        unsafe { self.location.as_mut().copy_from_slice(buffer) }
    }

    fn write(&self, buffer: &mut [u8], offset: usize) {
        buffer.copy_from_slice(unsafe { self.location.as_ref() });
    }
}
