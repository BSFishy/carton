//! TODO: document this

use std::fmt;

/// TODO: document this
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    /// TODO: document this
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    /// TODO: document this
    pub fn with_x(x: f32) -> Self {
        Self {
            x,
            y: 0f32,
        }
    }

    /// TODO: document this
    pub fn with_y(y: f32) -> Self {
        Self {
            x: 0f32,
            y
        }
    }
}

impl Pos {
    /// TODO: document this
    pub fn x(&self) -> f32 {
        self.x
    }

    /// TODO: document this
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    /// TODO: document this
    pub fn y(&self) -> f32 {
        self.y
    }

    /// TODO: document this
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self::new(0f32, 0f32)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
