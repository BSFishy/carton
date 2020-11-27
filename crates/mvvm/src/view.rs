//! TODO: document this

use crate::widget::Widget;

/// TODO: document this
pub trait View {
    /// TODO: document this
    type Output: View;

    /// TODO: document this
    fn build(&self) -> Self::Output;
}

/// TODO: document this
#[derive(Debug, Clone)]
pub struct WidgetView<T: Widget> {
    widget: T,
}

impl <T: Widget> WidgetView<T> {
    /// TODO: document this
    pub fn new(widget: T) -> Self {
        Self {
            widget,
        }
    }
}

impl <T: Widget> WidgetView<T> {
    /// TODO: document this
    pub fn get_widget(&self) -> &T {
        &self.widget
    }
}

impl <T: Widget> View for WidgetView<T> where T: Clone {
    type Output = Self;

    fn build(&self) -> Self {
        self.clone()
    }
}

// /// TODO: document this
// #[derive(Debug, Hash)]
// pub struct BuildCtx {
//
// }
//
// impl BuildCtx {
//     /// TODO: document this
//     pub fn bind(&mut self) -> &mut Self {
//         self
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
