//! TODO: document this

use crate::{ffi, connection::Connection, window::{WindowTrait, Builder}};

// use crate::ffi;
// use crate::connection::Connection;
//
// use super::WindowTrait;
// use crate::window::Builder;

use std::ptr;

pub struct X11Window {
    #[allow(unused)]
    window: ffi::xcb_window_t,
    // inner: inner::WindowInner<>,
}

// impl X11Window {
// }

impl WindowTrait for X11Window {
    fn new(connection: &Connection, builder: Builder) -> Self {
        unsafe {
            let window = ffi::xcb_generate_id(connection.connection);
            ffi::xcb_create_window(connection.connection,
                ffi::XCB_COPY_FROM_PARENT as u8,
                window,
                (*connection.screen).root,
                builder.attr.position().x() as i16, builder.attr.position().y() as i16,
                builder.attr.size().width() as u16, builder.attr.size().height() as u16,
                1,
                ffi::xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*connection.screen).root_visual,
                0, ptr::null());

            Self {
                window,
            }
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
