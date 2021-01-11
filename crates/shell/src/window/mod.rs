//! TODO: document this

use carton_common::{config_mod, config_type, prelude::{Size, Pos}};

use crate::connection::Connection;

mod inner;

/// TODO: document this
pub trait WindowTrait {
    /// TODO: document this
    fn builder(connection: &Connection) -> Builder {
        Builder::new(connection)
    }

    /// TODO: document this
    fn new(connection: &Connection, builder: Builder) -> Self;
}

config_mod!(pub(crate) platform, "x11" => x11);

config_type! {
    /// TODO: document this
    pub platform for Window, "x11" => x11::X11Window
}

/// TODO: document thisault for Attributes {
    fn default() -> Self {
        Self {
            position: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Debug, Default, Copy, Clone)]
pub struct Attributes {
    position: Pos,
    size: Size,
}

impl Attributes {
    /// TODO: document this
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// TODO: document this
    pub fn with_position(position: Pos) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    /// TODO: document this
    pub fn with_size(size: Size) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    /// TODO: document this
    pub fn with_position_and_size(position: Pos, size: Size) -> Self {
        Self {
            position,
            size,
        }
    }
}

impl Attributes {
    /// TODO: document this
    pub fn position(&self) -> &Pos {
        &self.position
    }

    /// TODO: document this
    pub fn set_position(&mut self, position: Pos) {
        self.position = position;
    }

    /// TODO: document this
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// TODO: document this
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }
}

// impl Default for Attributes {
//     fn default() -> Self {
//         Self {
//             position: Default::default(),
//             size: Default::default(),
//         }
//     }
// }

/// TODO: document this
#[derive(Debug)]
pub struct Builder<'c> {
    connection: &'c Connection,
    pub(crate) attr: Attributes,
}

impl<'c> Builder<'c> {
    /// TODO: document this
    pub fn new(connection: &'c Connection) -> Self {
        Self {
            connection,
            attr: Attributes::new(),
        }
    }
}

impl<'c> Builder<'c> {
    /// TODO: document this
    pub fn x(&mut self, x: f32) -> &mut Self {
        self.attr.position().set_x(x);
        self
    }

    /// TODO: document this
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.attr.position().set_y(y);
        self
    }

    /// TODO: document this
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.attr.size().set_width(width);
        self
    }

    /// TODO: document this
    pub fn height(&mut self, height: f32) -> &mut Self {
        self.attr.size().set_height(height);
        self
    }

    /// TODO: document this
    pub fn build(self) -> Window {
        Window::new(self.connection, self)
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4)
//     }
// }
