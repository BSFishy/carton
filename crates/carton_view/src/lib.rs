pub trait View {
    fn build(&self) -> Box<dyn View>;

    // TODO(BSFishy): implement this set_state function
    // fn set_state(&self, callback: Fn()) {
    //     callback();
    //     self.build();
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
