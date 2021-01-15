mod format;
mod plain;
mod to_string;

pub use self::{format::format, plain::Plain};

#[doc(hidden)]
pub mod __export {
    pub use stylish_core::format_args;
}
