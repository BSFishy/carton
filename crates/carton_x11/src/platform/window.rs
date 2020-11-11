use carton_common::{HasTitle, HasPosition, HasSize, Point, Size};
use carton_window::Window;
use carton_view::View;

use std::ffi::{c_void};
use libc::free;
use super::ffi::*;
use std::thread::sleep;
use std::time::Duration;
use std::ptr::{null, null_mut};

pub struct X11Window {
    connection: *mut XCBConnection,
    window: XCBWindow,
}

impl X11Window {
    pub(crate) unsafe fn new(connection: *mut XCBConnection) -> Self {
        let setup = xcb_get_setup(connection);
        let iter = xcb_setup_roots_iterator(setup);
        let screen = iter.data;

        let window = xcb_generate_id(connection);
        let value_list: [u32; 1] = [XCBEventMask::StructureNotify as u32];
        xcb_create_window(connection,
                          XCB_COPY_FROM_PARENT,
                          window,
                          (*screen).root,
                          0, 0,
                          150, 150,
                          10,
                          XCBWindowClass::InputOutput as u16,
                          (*screen).root_visual,
                          XCBCW::EventMask as u32, value_list.as_ptr() as *const c_void);

        // xcb_map_window(connection, window);

        Self { connection, window }
    }
}

impl X11Window {
    fn get_geometry(&self) -> *mut XCBGetGeometryReply {
        unsafe {
            let geom_cookie = xcb_get_geometry(self.connection, self.window);
            xcb_get_geometry_reply(self.connection, geom_cookie, null_mut())
        }
    }
}

impl Window for X11Window {
    fn show(&self) {
        unsafe {
            xcb_map_window(self.connection, self.window);

            xcb_flush(self.connection);

            let event = xcb_wait_for_event(self.connection);
            if let XCBEventType::ConfigureNotify = XCBEventType::from((*event).response_type & !0x80) {
                let event = event as *mut XCBConfigureNotifyEvent;

                // println!("Got a configure notify event {}x{}, ({}, {})", (*event).width, (*event).height, (*event).x, (*event).y);
            } else {
                // println!("Got no configure notify: {}", (*event).response_type);
            }

            free(event as *mut c_void);
        }
    }

    fn hide(&self) {
        unsafe {
            xcb_unmap_window(self.connection, self.window);

            xcb_flush(self.connection);
        }
    }

    fn get_body(&self) -> Box<dyn View> {
        unimplemented!()
    }

    fn set_body(&mut self, _view: Box<dyn View>) {
        unimplemented!()
    }

    fn run(&self) {
        sleep(Duration::from_secs(5));

        unsafe {
            xcb_disconnect(self.connection);
        }
    }

    fn run_async(&self) {
        unimplemented!()
    }
}

impl HasTitle for X11Window {
    fn get_title(&self) -> String {
        unimplemented!()
    }

    fn set_title(&mut self, title: String) {
        unsafe {
            xcb_change_property(self.connection,
                                XCBPropMode::Replace as u8,
                                self.window,
                                XCBAtomEnum::WmName as u32,
                                XCBAtomEnum::String as u32,
                                8,
                                title.len() as u32,
                                title.as_ptr() as *const c_void);

            // xcb_map_window(self.connection, self.window);
            //
            // xcb_flush(self.connection);
        }
    }
}

impl HasPosition for X11Window {
    fn get_position(&self) -> Point {
        let geom = self.get_geometry();
        let position: Point;

        unsafe {
            position = Point::new((*geom).x as f32, (*geom).y as f32);
            free(geom as *mut c_void);
        }

        position
    }

    fn set_position(&mut self, position: Point) {
        let pos: [u32; 2] = [position.get_x() as u32, position.get_y() as u32];

        unsafe {
            xcb_configure_window(self.connection,
                                 self.window,
                                 XCBConfigWindow::X as u16 | XCBConfigWindow::Y as u16,
                                 pos.as_ptr() as *const c_void);

            // xcb_map_window(self.connection, self.window);
            //
            // xcb_flush(self.connection);
        }
    }
}

impl HasSize for X11Window {
    fn get_size(&self) -> Size {
        let geom = self.get_geometry();
        let size: Size;

        unsafe {
            size = Size::new((*geom).width as u32, (*geom).height as u32);
            free(geom as *mut c_void);
        }

        size
    }

    fn set_size(&mut self, size: Size) {
        let s: [u32; 2] = [size.get_width() as u32, size.get_height() as u32];

        unsafe {
            xcb_configure_window(self.connection,
                                 self.window,
                                 XCBConfigWindow::Width as u16 | XCBConfigWindow::Height as u16,
                                 s.as_ptr() as *const c_void);

            // xcb_map_window(self.connection, self.window);
            //
            // xcb_flush(self.connection);
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
