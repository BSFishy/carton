use carton_window::Window;

pub trait Provider {
    fn new() -> Self where Self: Sized;

    fn create_window(&self) -> Box<dyn Window>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
