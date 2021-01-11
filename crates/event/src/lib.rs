//! TODO: document this

/// TODO: document this
pub trait EventHandler<T> {
    /// TODO: document this
    fn handle(&self, event: &T);
}

/// TODO: document this
pub struct EventPropagator<T> {
    handler: Option<fn(&T)>,
}

impl <T> EventPropagator<T> {
    /// TODO: document this
    pub fn new() -> Self {
        Self {
            handler: None,
        }
    }

    /// TODO: document this
    pub fn with_handler(handler: fn(&T)) -> Self {
        Self {
            handler: Some(handler),
        }
    }
}

impl <T> EventPropagator<T> {
    /// TODO: document this
    pub fn get_handler(&self) -> &Option<fn(&T)> {
        &self.handler
    }

    /// TODO: document this
    pub fn has_handler(&self) -> bool {
        self.handler.is_some()
    }

    /// TODO: document this
    pub fn set_handler(&mut self, handler: Option<fn(&T)>) {
        self.handler = handler
    }

    /// TODO: document this
    pub fn set_handler_some(&mut self, handler: fn(&T)) {
        self.handler = Some(handler)
    }

    /// TODO: document this
    pub fn set_handler_none(&mut self) {
        self.handler = None
    }
}

impl <T> EventHandler<T> for EventPropagator<T> {
    fn handle(&self, event: &T) {
        if let Some(handler) = self.handler {
            handler(event)
        }
    }
}

impl <T> Default for EventPropagator<T> {
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
