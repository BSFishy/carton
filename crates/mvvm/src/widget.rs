//! TODO: document this

/// TODO: document this
pub trait Widget {
    fn layout(&self);

    fn paint(&self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
