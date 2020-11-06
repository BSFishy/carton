use super::Drawer;

pub trait Drawable {
    fn draw<T: Drawer>(&self, drawer: &T);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
