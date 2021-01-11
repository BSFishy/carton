//! TODO: document this

use std::fmt;

use carton_mvvm::View;

/// TODO: document this
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Text {
    content: String,
}

impl Text {
    /// TODO: document this
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// TODO: document this
    pub fn with_content(content: String) -> Self {
        Self {
            content,
        }
    }
}

impl View for Text {
    fn layout(&self) {
        unimplemented!()
    }

    fn draw(&self) {
        unimplemented!()
    }

    fn update(&self) {
        unimplemented!()
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.content.fmt(f)
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
