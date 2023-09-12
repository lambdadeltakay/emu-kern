use crate::driver::EmuRsDriver;

pub trait EmuRsDisk: EmuRsDriver {
    fn read(&self, buffer: &[u8], offset: usize);
    fn write(&self, buffer: &mut [u8], offset: usize);
}
