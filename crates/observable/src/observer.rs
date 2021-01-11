//! TODO: document this

#[derive(Debug, Clone, Hash)]
pub struct Observer<T> {
    value: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
