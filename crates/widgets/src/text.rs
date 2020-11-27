//! TODO: document this

use carton_mvvm::widget::Widget;

/// TODO: document this
pub struct Text;

impl Widget for Text {
    fn layout(&self) {
        unimplemented!()
    }

    fn paint(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
