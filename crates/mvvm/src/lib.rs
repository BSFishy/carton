//! TODO: document this

pub mod model;
pub mod view;
pub mod view_model;

pub mod scene;
pub mod widget;

/// TODO: document this
pub mod prelude {
    pub use crate::model::Model;
    pub use crate::view::View;
    pub use crate::view_model::ViewModel;

    pub use crate::scene::Scene;
    pub use crate::widget::Widget;
}
