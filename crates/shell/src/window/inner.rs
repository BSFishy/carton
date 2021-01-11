use carton_mvvm::View;

#[derive(Debug, Default, Hash)]
pub(crate) struct WindowInner<T> where T: View {
    root_view: T,
}

impl<T: View> WindowInner<T> {
    pub(crate) fn new(root_view: T) -> WindowInner<T> {
        WindowInner {
            root_view,
        }
    }

    pub(crate) fn root_view(&self) -> &T {
        &self.root_view
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
