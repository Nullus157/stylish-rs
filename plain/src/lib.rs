#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod format;
mod plain;
#[cfg(feature = "alloc")]
mod to_string;

pub use self::plain::Plain;
#[cfg(feature = "alloc")]
pub use self::{format::format, to_string::ToPlainString};

#[doc(hidden)]
pub mod __export {
    pub use stylish_core::format_args;
}
