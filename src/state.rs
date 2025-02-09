use crate::shape::shape::Shape;

/// Contains all the state that will be used in the ui / widgets.
#[derive(Debug)]
pub struct State {
    pub shapes: Vec<Shape>
}

impl State {
    /// Creates new state.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new()
        }
    }
}
