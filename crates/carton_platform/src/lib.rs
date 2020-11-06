use carton_provider::Provider;

pub trait Platform {
    fn is_supported(&self) -> bool;

    fn get_provider<T: Provider>(&self) -> T where Self: Sized {
        panic!("This platform is not supported")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
