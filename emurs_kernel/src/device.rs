use alloc::vec::Vec;
use core::ops::RangeInclusive;
use tinyvec::{ArrayVec, TinyVec};

use crate::mem::EmuRsMemoryRange;

pub struct EmuRsDeviceTable {
    devices: Vec<EmuRsDevice>,
}

impl EmuRsDeviceTable {
    pub fn register(&mut self, dev: EmuRsDevice) {
        self.devices.push(dev);
    }
}

pub struct EmuRsDevice {
    pub memory: TinyVec<[EmuRsMemoryRange; 2]>,
}
