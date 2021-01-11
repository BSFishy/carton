//! TODO: document this

use carton_mvvm::View;

/// TODO: document this
#[derive(Debug, Hash)]
pub struct Window<T> where T: View {
    root_view: T,
}

impl<T: View> Window<T> {
    /// TODO: document this
    pub fn new(view: T) -> Window<T> {
        Window {
            root_view: view,
        }
    }

    /// TODO: document this
    pub fn root_view(&self) -> &T {
        &self.root_view
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
