//! TODO: document this

// use carton_mvvm::{
//     widget::Widget,
//     view::{View}
// };

// use std::{any, fmt};

pub struct Flex;

// /// TODO: document this
// #[derive(Clone)]
// pub struct Flex<T> where T: View + Clone {
//     view: T,
// }
//
// impl <T> Flex<T> where T: View + Clone {
//     /// TODO: document this
//     pub fn new(function: fn() -> T) -> Self {
//         Self {
//             view: function(),
//         }
//     }
//
//     pub fn with_view(view: T) -> Self {
//         Self {
//             view,
//         }
//     }
// }
//
// impl <T> Widget for Flex<T> where T: View + Clone {
//     fn layout(&self) {
//         unimplemented!()
//     }
//
//     fn paint(&self) {
//         unimplemented!()
//     }
// }
//
// impl <T> fmt::Debug for Flex<T> where T: View + Clone + fmt::Debug {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct(&*format!("Flex<{}>", any::type_name::<T>()))
//             .field("view", &self.view)
//             .finish()
//     }
// }
//
// impl <T> fmt::Display for Flex<T> where T: View + Clone + fmt::Display {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.view.fmt(f)
//     }
// }
//
// impl <T> Default for Flex<T> where T: View + Clone + Default {
//     fn default() -> Self {
//         Self::new(|| T::default())
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
