//! TODO: document this

use std::fmt;

/// TODO: document this
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// TODO: document this
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}

impl Color {
    /// TODO: document this
    pub fn r(&self) -> u8 {
        self.r
    }

    /// TODO: document this
    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    /// TODO: document this
    pub fn g(&self) -> u8 {
        self.g
    }

    /// TODO: document this
    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    /// TODO: document this
    pub fn b(&self) -> u8 {
        self.b
    }

    /// TODO: document this
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
