//! TODO: document this

use carton_common::{config_mod, config_type};

use crate::window::{Builder};

/// TODO: document this
pub trait ConnectionTrait {
    /// TODO: document this
    fn new() -> Self;

    /// TODO: document this
    fn create_window(&self) -> Builder;
}

config_mod!(pub(crate) platform, "x11" => x11);

config_type! {
    /// TODO: document this
    pub platform for Connection, "x11" => x11::X11Connection
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
