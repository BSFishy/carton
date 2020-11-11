use std::ops;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Point {
    pub const ZERO: Point = Point { x: 0f32, y: 0f32 };
    pub const ONE: Point = Point { x: 1f32, y: 1f32 };

    pub const UP: Point = Point { x: 0f32, y: 1f32 };
    pub const DOWN: Point = Point { x: 0f32, y: -1f32 };
    pub const RIGHT: Point = Point { x: 1f32, y: 0f32 };
    pub const LEFT: Point = Point { x: -1f32, y: 0f32 };
}

impl Point {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl Point {
    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
}

impl Point {
    pub fn get_length(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn get_normalized(&self) -> Self {
        let len = self.get_length();
        Self { x: self.x / len, y: self.y / len }
    }
}

impl Point {
    pub fn normalize(&mut self) -> &mut Self {
        let len = self.get_length();
        self.x /= len;
        self.y /= len;
        self
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0f32, 0f32)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

macro_rules! primitive_ops_impl {
    ($type:ident, $rhs:ty, $func:ident, $op:tt) => {
        impl ops::$type<$rhs> for Point {
            type Output = Point;

            fn $func(self, rhs: $rhs) -> Self::Output {
                Self { x: self.x $op rhs as f32, y: self.y $op rhs as f32 }
            }
        }
    }
}

macro_rules! ops_impl {
    ($type:ident, $func:ident, $op:tt) => {
        impl ops::$type for Point {
            type Output = Point;

            fn $func(self, rhs: Self) -> Self::Output {
                Self { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }

        primitive_ops_impl!($type, u8, $func, $op);
        primitive_ops_impl!($type, u16, $func, $op);
        primitive_ops_impl!($type, u32, $func, $op);

        primitive_ops_impl!($type, i8, $func, $op);
        primitive_ops_impl!($type, i16, $func, $op);
        primitive_ops_impl!($type, i32, $func, $op);

        primitive_ops_impl!($type, f32, $func, $op);
    }
}

macro_rules! primitive_assign_ops_impl {
    ($type:ident, $rhs:ty, $func:ident, $op:tt) => {
        impl ops::$type<$rhs> for Point {
            fn $func(&mut self, rhs: $rhs) {
                self.x $op rhs as f32;
                self.y $op rhs as f32;
            }
        }
    }
}

macro_rules! assign_ops_impl {
    ($type:ident, $func:ident, $op:tt) => {
        impl ops::$type for Point {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }

        primitive_assign_ops_impl!($type, u8, $func, $op);
        primitive_assign_ops_impl!($type, u16, $func, $op);
        primitive_assign_ops_impl!($type, u32, $func, $op);

        primitive_assign_ops_impl!($type, i8, $func, $op);
        primitive_assign_ops_impl!($type, i16, $func, $op);
        primitive_assign_ops_impl!($type, i32, $func, $op);

        primitive_assign_ops_impl!($type, f32, $func, $op);
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
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
