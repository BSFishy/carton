use super::Point;

pub trait HasPosition {
    fn get_position(&self) -> Point;

    fn set_position(&mut self, position: Point);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
