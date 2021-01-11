//! TODO: document this

pub mod draw_call;
pub mod drawer;

/// TODO: document this
pub mod prelude {
    pub use crate::draw_call::DrawCall;
    pub use crate::drawer::Drawer;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
