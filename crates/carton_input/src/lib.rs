pub trait Toggleable {
    fn get_state(&self) -> bool;

    fn down(&self);

    fn is_down(&self) -> bool;

    fn is_now_down(&self) -> bool;

    fn up(&self);

    fn is_up(&self) -> bool;

    fn is_now_up(&self) -> bool;
}

mod mouse;
pub use mouse::*;

mod keyboard;
pub use keyboard::*;

mod touch;
pub use touch::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
