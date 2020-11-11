use carton_provider::Provider;
use carton_window::Window;

use super::ffi::*;
use std::ptr::{null, null_mut};
use crate::X11Window;

pub struct X11Provider {
    connection: *mut XCBConnection,
}

impl Provider for X11Provider {
    fn new() -> Self where Self: Sized {
        let connection: *mut XCBConnection;

        unsafe {
            connection = xcb_connect(null(), null_mut());
        }

        Self { connection }
    }

    fn create_window(&self) -> Box<dyn Window> {
        unsafe {
            Box::new(X11Window::new(self.connection))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
