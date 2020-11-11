pub trait HasTitle {
    fn get_title(&self) -> String;

    fn set_title(&mut self, title: String);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
