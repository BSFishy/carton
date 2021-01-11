//! TODO: document this

/// TODO: document this
pub trait View {
    /// TODO: document this
    fn layout(&self);

    /// TODO: document this
    fn draw(&self);

    /// TODO: document this
    fn update(&self);
}

impl<T> View for Option<T> where T: View {
    fn layout(&self) {
        if let Some(view) = self {
            view.layout()
        }
    }

    fn draw(&self) {
        if let Some(view) = self {
            view.draw()
        }
    }

    fn update(&self) {
        if let Some(view) = self {
            view.update()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
