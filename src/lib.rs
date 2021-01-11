//! TODO: document this

#[doc(inline)]
pub use carton_mvvm as mvvm;

#[doc(inline)]
pub use crate::mvvm::View;

#[doc(inline)]
pub use carton_shell as shell;

#[cfg(feature = "widgets")]
#[doc(inline)]
pub use carton_widgets as widgets;

// /// TODO: document this
// pub mod prelude {
//     pub use crate::mvvm::prelude::*;
//     pub use crate::shell::prelude::*;
//
//     #[cfg(feature = "widgets")]
//     pub use crate::widgets::prelude::*;
// }
