//! TODO: document this

mod observable;
mod observer;
mod binding;

/// TODO: document this
pub mod prelude {
    pub use crate::observable::Observable;
    pub use crate::observer::Observer;
    pub use crate::binding::Binding;
}
