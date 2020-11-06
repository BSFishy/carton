pub struct NoPlatformError;

impl NoPlatformError {
    pub fn get_message() -> &'static str {
        "No suitable platform found"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
