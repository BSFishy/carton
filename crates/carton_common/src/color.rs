use std::ops;

#[derive(Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
}

impl Color {
    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }
}

impl Color {
    pub fn set_r(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self
    }

    pub fn set_g(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self
    }

    pub fn set_b(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

macro_rules! ops_impl {
    ($type:ident, $func:ident, $op:tt) => {
        impl ops::$type for Color {
            type Output = Color;

            fn $func(self, rhs: Self) -> Self::Output {
                Self { r: self.r $op rhs.r, g: self.g $op rhs.g, b: self.b $op rhs.b }
            }
        }

        impl ops::$type<u8> for Color {
            type Output = Color;

            fn $func(self, rhs: u8) -> Self::Output {
                Self { r: self.r $op rhs, g: self.g $op rhs, b: self.b $op rhs }
            }
        }
    }
}

macro_rules! assign_ops_impl {
    ($type:ident, $func:ident, $op:tt) => {
        impl ops::$type for Color {
            fn $func(&mut self, rhs: Self) {
                self.r $op rhs.r;
                self.g $op rhs.g;
                self.b $op rhs.b;
            }
        }

        impl ops::$type<u8> for Color {
            fn $func(&mut self, rhs: u8) {
                self.r $op rhs;
                self.g $op rhs;
                self.b $op rhs;
            }
        }
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
        macro_rules! new_test {
            ($r:expr, $g:expr, $b:expr) => {
                assert_eq!(Color::new($r, $g, $b), Color { r: $r, b: $b, g: $g }, "the new function does not work")
            }
        }

        new_test!(0, 0, 0);
        new_test!(1, 0, 0);
        new_test!(0, 1, 0);
        new_test!(1, 1, 0);
        new_test!(0, 0, 1);
        new_test!(1, 0, 1);
        new_test!(0, 1, 1);
        new_test!(1, 1, 1);
    }

    #[test]
    fn default_is_zeroed() {
        assert_eq!(Color::default(), Color { r: 0, g: 0, b: 0 }, "the default value is not zeroed")
    }

    macro_rules! get_test {
        ($set_func:ident, $get_func:ident, $val:expr) => {
            get_test!($set_func, $get_func, 0, 0, 0, $val);
        };
        ($set_func:ident, $get_func:ident, $r:expr, $g:expr, $b:expr, $val:expr) => {
            assert_eq!(Color::new($r, $g, $b).$set_func($val).$get_func(), $val, "{} does not return the correct value", stringify!($func));
        }
    }

    macro_rules! get_works {
        ($f_name:ident, $set_func:ident, $get_func:ident) => {
            #[test]
            fn $f_name() {
                get_test!($set_func, $get_func, 0);
                get_test!($set_func, $get_func, 1);

                get_test!($set_func, $get_func, 0, 0, 0, 0);
                get_test!($set_func, $get_func, 1, 0, 0, 0);
                get_test!($set_func, $get_func, 0, 1, 0, 0);
                get_test!($set_func, $get_func, 1, 1, 0, 0);
                get_test!($set_func, $get_func, 0, 0, 1, 0);
                get_test!($set_func, $get_func, 1, 0, 1, 0);
                get_test!($set_func, $get_func, 0, 1, 1, 0);
                get_test!($set_func, $get_func, 1, 1, 1, 0);

                get_test!($set_func, $get_func, 0, 0, 0, 1);
                get_test!($set_func, $get_func, 1, 0, 0, 1);
                get_test!($set_func, $get_func, 0, 1, 0, 1);
                get_test!($set_func, $get_func, 1, 1, 0, 1);
                get_test!($set_func, $get_func, 0, 0, 1, 1);
                get_test!($set_func, $get_func, 1, 0, 1, 1);
                get_test!($set_func, $get_func, 0, 1, 1, 1);
                get_test!($set_func, $get_func, 1, 1, 1, 1);
            }
        }
    }

    get_works!(get_r_works, set_r, get_r);
    get_works!(get_g_works, set_g, get_g);
    get_works!(get_b_works, set_b, get_b);
}
