//! TODO: document this

use crate::view::View;

/// TODO: document this
pub trait Scene {
    type RootView: View;

    fn build(&self) -> Self::RootView;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
