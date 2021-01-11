//! TODO: document this

use crate::ffi;

use super::ConnectionTrait;
use crate::window::Builder;

use std::ptr;

#[derive(Debug)]
pub struct X11Connection {
    pub(crate) connection: *mut ffi::xcb_connection_t,
    pub(crate) setup: *const ffi::xcb_setup_t,
    pub(crate) screen: *mut ffi::xcb_screen_t,
}

impl ConnectionTrait for X11Connection {
    fn new() -> Self {
        unsafe {
            let connection = ffi::xcb_connect(ptr::null(), ptr::null_mut());

            let setup = ffi::xcb_get_setup(connection);
            let iter = ffi::xcb_setup_roots_iterator(setup);
            let screen = iter.data;

            Self {
                connection,
                setup,
                screen,
            }
        }
    }

    fn create_window(&self) -> Builder {
        Builder::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
