//! TODO: document this

#[macro_use]
pub mod macros;

pub mod size;
pub mod pos;
pub mod color;

/// TODO: document this
pub mod prelude {
    pub use crate::size::*;
    pub use crate::pos::*;
    pub use crate::color::*;
}
