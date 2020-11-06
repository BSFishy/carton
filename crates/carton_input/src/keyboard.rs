use super::Toggleable;

pub enum KeyCode {

}

pub trait Keyboard {
    fn get_key<'a, T: Toggleable>(&self, key: KeyCode) -> &'a T;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
