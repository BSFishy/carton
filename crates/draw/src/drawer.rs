//! TODO: document this

use crate::draw_call::DrawCall;

/// TODO: document this
pub trait Drawer {
    /// TODO: document this
    fn draw(&self, call: DrawCall);

    /// TODO: document this
    fn draw_all(&self, calls: Vec<DrawCall>) {
        for call in calls {
            self.draw(call)
        }
    }

    /// TODO: document this
    fn flush(&self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
