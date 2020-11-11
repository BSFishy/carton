use std::ops;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Size {
    pub const ZERO: Size = Size { width: 0, height: 0 };
    pub const ONE: Size = Size { width: 1, height: 1 };
}

impl Size {
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl Size {
    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
}

impl Size {
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

macro_rules! primitive_ops_impl {
    ($type:ident, $rhs:ty, $func:ident, $op:tt) => {
        impl ops::$type<$rhs> for Size {
            type Output = Size;

            fn $func(self, rhs: $rhs) -> Self::Output {
                Self { width: self.width $op rhs as u32, height: self.height $op rhs as u32 }
            }
        }
    }
}

macro_rules! ops_impl {
    ($type:ident, $func:ident, $op:tt) => {
        impl ops::$type for Size {
            type Output = Size;

            fn $func(self, rhs: Self) -> Self::Output {
                Self { width: self.width $op rhs.width, height: self.height $op rhs.height }
            }
        }

        primitive_ops_impl!($type, u8, $func, $op);
        primitive_ops_impl!($type, u16, $func, $op);
        primitive_ops_impl!($type, u32, $func, $op);
    }
}

macro_rules! primitive_assign_ops_impl {
    ($type:ident, $rhs:ty, $func:ident, $op:tt) => {
        impl ops::$type<$rhs> for Size {
            fn $func(&mut self, rhs: $rhs) {
                self.width $op rhs as u32;
                self.height $op rhs as u32;
            }
        }
    }
}

macro_rules! assign_ops_impl {
    ($type:ident, $func:ident, $op:tt) => {
        impl ops::$type for Size {
            fn $func(&mut self, rhs: Self) {
                self.width $op rhs.width;
                self.height $op rhs.height;
            }
        }

        primitive_assign_ops_impl!($type, u8, $func, $op);
        primitive_assign_ops_impl!($type, u16, $func, $op);
        primitive_assign_ops_impl!($type, u32, $func, $op);
    }
}

ops_impl!(Add, add, +);
ops_impl!(Sub, sub, -);
ops_impl!(Mul, mul, *);
ops_impl!(Div, div, /);

assign_ops_impl!(AddAssign, add_assign, +=);
assign_ops_impl!(SubAssign, sub_assign, -=);
assign_ops_impl!(MulAssign, mul_assign, *=);
assign_ops_impl!(DivAssign, div_assign, /=);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(Size::new(0, 0), Size { width: 0, height: 0 }, "the new function does not create a proper size");
        assert_eq!(Size::new(1, 0), Size { width: 1, height: 0 }, "the new function does not create a proper size");
        assert_eq!(Size::new(0, 1), Size { width: 0, height: 1 }, "the new function does not create a proper size");
        assert_eq!(Size::new(1, 1), Size { width: 1, height: 1 }, "the new function does not create a proper size");
    }

    #[test]
    fn default_is_zeroed() {
        assert_eq!(Size::default(), Size { width: 0, height: 0 }, "the default value is not zeroed")
    }

    #[test]
    fn equatable() {
        assert_eq!(Size::new(0, 0), Size::new(0, 0), "the equality operator does not work");
        assert_eq!(Size::new(1, 0), Size::new(1, 0), "the equality operator does not work");
        assert_eq!(Size::new(0, 1), Size::new(0, 1), "the equality operator does not work");
        assert_eq!(Size::new(1, 1), Size::new(1, 1), "the equality operator does not work");

        assert_ne!(Size::new(0, 0), Size::new(0, 1), "the equality operator does not work");
        assert_ne!(Size::new(0, 0), Size::new(1, 0), "the equality operator does not work");
        assert_ne!(Size::new(0, 0), Size::new(1, 1), "the equality operator does not work");

        assert_ne!(Size::new(1, 0), Size::new(0, 0), "the equality operator does not work");
        assert_ne!(Size::new(1, 0), Size::new(0, 1), "the equality operator does not work");
        assert_ne!(Size::new(1, 0), Size::new(1, 1), "the equality operator does not work");

        assert_ne!(Size::new(0, 1), Size::new(0, 0), "the equality operator does not work");
        assert_ne!(Size::new(0, 1), Size::new(1, 0), "the equality operator does not work");
        assert_ne!(Size::new(0, 1), Size::new(1, 1), "the equality operator does not work");

        assert_ne!(Size::new(1, 1), Size::new(0, 0), "the equality operator does not work");
        assert_ne!(Size::new(1, 1), Size::new(0, 1), "the equality operator does not work");
        assert_ne!(Size::new(1, 1), Size::new(1, 0), "the equality operator does not work");
    }

    #[test]
    fn get_width_works() {
        assert_eq!(Size::new(0, 0).get_width(), 0, "get_width does not return the correct value");
        assert_eq!(Size::new(1, 0).get_width(), 1, "get_width does not return the correct value");
        assert_eq!(Size::new(0, 1).get_width(), 0, "get_width does not return the correct value");
        assert_eq!(Size::new(1, 1).get_width(), 1, "get_width does not return the correct value");
    }

    #[test]
    fn get_height_works() {
        assert_eq!(Size::new(0, 0).get_height(), 0, "get_height does not return the correct value");
        assert_eq!(Size::new(1, 0).get_height(), 0, "get_height does not return the correct value");
        assert_eq!(Size::new(0, 1).get_height(), 1, "get_height does not return the correct value");
        assert_eq!(Size::new(1, 1).get_height(), 1, "get_height does not return the correct value");
    }
}
