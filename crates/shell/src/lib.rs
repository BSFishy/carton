//! TODO: document this

pub(crate) mod ffi;

pub mod connection;
pub mod window;

/// TODO: document this
pub mod prelude {
    pub use crate::connection::Connection;
    pub use crate::window::Window;
}
