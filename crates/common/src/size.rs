//! TODO: document this

use std::fmt;

/// TODO: document this
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    /// TODO: document this
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    /// TODO: document this
    pub fn with_width(width: f32) -> Self {
        Self {
            width,
            height: 0f32,
        }
    }

    /// TODO: document this
    pub fn with_height(height: f32) -> Self {
        Self {
            width: 0f32,
            height,
        }
    }
}

impl Size {
    /// TODO: document this
    pub fn width(&self) -> f32 {
        self.width
    }

    /// TODO: document this
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    /// TODO: document this
    pub fn height(&self) -> f32 {
        self.height
    }

    /// TODO: document this
    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(0f32, 0f32)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
