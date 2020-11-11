use super::Size;

pub trait HasSize {
    fn get_size(&self) -> Size;

    fn set_size(&mut self, size: Size);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
