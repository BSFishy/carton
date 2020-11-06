use super::Toggleable;

pub enum MouseButton {

}

pub trait Mouse {
    fn get_button<'a, T: Toggleable>(&self, button: MouseButton) -> &'a T;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
