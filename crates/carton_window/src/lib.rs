use carton_common::{ HasTitle, HasPosition, HasSize };
use carton_view::View;

pub trait Window: HasTitle + HasPosition + HasSize {
    // fn new() -> Self where Self: Sized;

    fn show(&self);

    fn hide(&self);

    fn get_body(&self) -> Box<dyn View>;

    fn set_body(&mut self, view: Box<dyn View>);

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
