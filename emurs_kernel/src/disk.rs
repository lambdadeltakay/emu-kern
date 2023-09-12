use crate::device::EmuRsDevice;
use crate::driver::EmuRsDriver;
use core::ptr::NonNull;
use crate::driver::EmuRsDriverPreference;

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
        return "Dummy Disk";
    }

    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {}

    fn get_preference(&self) -> EmuRsDriverPreference {
        return EmuRsDriverPreference::Fallback;
    }
}

impl EmuRsDiskDriver for EmuRsDummyDiskDriver {
    fn write(&mut self, buffer: &[u8], offset: usize) {}
    fn read(&mut self, buffer: &mut [u8], offset: usize) {}
}

/// A disk that just points somewhere in memory. Useful for the GBA save slot
pub trait EmuRsMemoryDisk {
    fn get_memory(&self) -> &mut [u8];
}

impl<'owner, T: EmuRsMemoryDisk + EmuRsDriver> EmuRsDiskDriver for T {
    fn write(&mut self, buffer: &[u8], offset: usize) {
        let start = offset;
        let end = buffer.len() + offset;

        self.get_memory()[start..end].copy_from_slice(buffer);
    }

    fn read(&mut self, buffer: &mut [u8], offset: usize) {
        buffer.copy_from_slice(&self.get_memory()[offset..buffer.len() + offset]);
    }
}
