pub use crate::emurs_main;
pub use crate::mem::EmuRsMemoryKind;
pub use crate::mem::EmuRsMemoryPermission;
pub use crate::mem::EmuRsMemoryTable;
pub use crate::mem::EmuRsMemoryTableEntry;

// Needed for embedded targets
#[cfg(feature = "embedded")]
pub use crate::error::panic_handler;

// Rexport these for common use
pub use nalgebra;
pub use tinyvec;
pub use bitfield;
pub use lock_api;
// FIXME: Make own lock implementation
pub use spin;