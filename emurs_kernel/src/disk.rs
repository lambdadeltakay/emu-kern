use core::intrinsics::size_of;



use crate::device::EmuRsDevice;
use crate::driver::EmuRsDriver;
use crate::driver::EmuRsDriverPreference;
use crate::error::EmuRsError;
use crate::error::EmuRsErrorReason;




/// The disk implementation for filesystems to write and read
/// IMPORTANT: Disks MUST return failure if they cannot fill the entire buffer. This is a hard requirement
pub trait EmuRsDiskDriver: EmuRsDriver {
    fn write(&mut self, _buffer: &[u8], _offset: usize) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn read(&mut self, _buffer: &mut [u8], _offset: usize) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn get_sector_size(&mut self) -> usize;
    fn get_total_size(&mut self) -> usize;
}

/// A disk that just points somewhere in memory. Useful for the GBA save slot
pub trait EmuRsMemoryDisk {
    fn get_memory(&self) -> &mut [u8];
}

impl<'owner, T: EmuRsMemoryDisk + EmuRsDriver> EmuRsDiskDriver for T {
    fn write(&mut self, buffer: &[u8], offset: usize) -> Result<(), EmuRsError> {
        let start = offset;
        let end = buffer.len() + offset;

        if self.get_memory().len() < start || self.get_memory().len() < end {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::EndOfDiskHit,
            });
        }

        self.get_memory()[start..end].copy_from_slice(buffer);
        return Ok(());
    }

    fn read(&mut self, buffer: &mut [u8], offset: usize) -> Result<(), EmuRsError> {
        let start = offset;
        let end = buffer.len() + offset;

        if self.get_memory().len() < start || self.get_memory().len() < end {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::EndOfDiskHit,
            });
        }

        buffer.copy_from_slice(&self.get_memory()[offset..buffer.len() + offset]);
        return Ok(());
    }

    fn get_sector_size(&mut self) -> usize {
        // FIXME: This is probably not so smart
        return size_of::<usize>();
    }

    fn get_total_size(&mut self) -> usize {
        return self.get_memory().len();
    }
}

/// A disk that simply points to a internal blob of bytes. Useful for using [include_bytes] to store games inline
pub struct EmuRsInternalDisk<'owner> {
    pub data: &'owner [u8],
}

impl<'owner> EmuRsDriver for EmuRsInternalDisk<'owner> {
    fn name(&self) -> &'static str {
        return "Internal Disk";
    }

    fn get_preference(&mut self) -> EmuRsDriverPreference {
        todo!()
    }

    fn get_claimed(&mut self) -> EmuRsDevice {
        todo!()
    }
}

impl<'owner> EmuRsDiskDriver for EmuRsInternalDisk<'owner> {
    fn read(&mut self, buffer: &mut [u8], offset: usize) -> Result<(), EmuRsError> {
        let start = offset;
        let end = buffer.len() + offset;

        if self.data.len() < start || self.data.len() < end {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::EndOfDiskHit,
            });
        }

        buffer.copy_from_slice(&self.data[offset..offset + buffer.len()]);
        return Ok(());
    }

    fn get_sector_size(&mut self) -> usize {
        // FIXME: This is probably not so smart
        return size_of::<usize>();
    }

    fn get_total_size(&mut self) -> usize {
        return self.data.len();
    }
}
