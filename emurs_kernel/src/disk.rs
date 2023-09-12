use crate::device::EmuRsDevice;
use crate::driver::EmuRsDriver;
use core::ptr::NonNull;

/// The disk implementation for filesystems to write and read
pub trait EmuRsDiskDriver: EmuRsDriver {
    /// TODO: Allow failure
    fn write(&mut self, buffer: &[u8], offset: usize);
    /// TODO: Allow failure
    fn read(&mut self, buffer: &mut [u8], offset: usize);
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
    fn write(&mut self, buffer: &[u8], offset: usize) {}
    fn read(&mut self, buffer: &mut [u8], offset: usize) {}
}

/// A disk that just points somewhere in memory. Useful for the GBA save slot
pub struct EmuRsMemoryDisk<'owner> {
    location: &'owner mut [u8],
}

impl<'owner> EmuRsDriver for EmuRsMemoryDisk<'owner> {
    fn name(&self) -> &str {
        return "Memory Disk";
    }

    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {}
}

impl<'owner> EmuRsMemoryDisk<'owner> {
    /// TODO: Perhaps rethink using NonNull
    pub fn new(location: &'owner mut [u8]) -> Self {
        return Self { location };
    }
}

impl<'owner> EmuRsDiskDriver for EmuRsMemoryDisk<'owner> {
    fn write(&mut self, buffer: &[u8], offset: usize) {
        let start = offset;
        let end = buffer.len() + offset;

        unsafe {
            self.location[start..end].copy_from_slice(buffer);
        }
    }

    fn read(&mut self, buffer: &mut [u8], offset: usize) {
        buffer.copy_from_slice(unsafe { &self.location[offset..buffer.len() + offset] });
    }
}
