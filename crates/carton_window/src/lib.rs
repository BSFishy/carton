use carton_common::{ Size, Point };
use carton_view::View;

pub trait Window {
    fn get_title(&self) -> String;

    fn set_title(&self, title: String) -> Self;

    fn get_size(&self) -> Size;

    fn set_size(&self, size: Size) -> Self;

    fn get_position(&self) -> Point;

    fn set_position(&self, position: Point) -> Self;

    fn get_view(&self) -> Box<dyn View>;

    fn set_view(&self, view: Box<dyn View>) -> Self;

    fn run(&self);

    fn run_async(&self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
