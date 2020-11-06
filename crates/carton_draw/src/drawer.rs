use carton_common::{ Color, Point, Size };

pub trait Drawer {
    fn set_color(&self, color: Color) -> Self;

    fn draw_point(&self, position: Point) -> Self;

    fn draw_line(&self, first: Point, second: Point) -> Self;

    fn draw_rectangle(&self, position: Point, size: Size) -> Self;

    fn fill_rectangle(&self, position: Point, size: Size) -> Self;

    fn draw_arc(&self, position: Point, size: Size, angle1: u16, angle2: u16) -> Self;

    fn fill_arc(&self, position: Point, size: Size, angle1: u16, angle2: u16) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
