//! TODO: document this

pub type EventHandler<T> = dyn Fn(T) + Send + 'static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
