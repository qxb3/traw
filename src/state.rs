use ratatui::layout::Position;

use crate::shape::shape::Shape;

/// Contains all the state that will be used in the ui / widgets.
#[derive(Debug)]
pub struct State {
    /// Whether the user is currently dragging.
    pub dragging: bool,

    /// Where the start position of the dragging starts.
    pub start_drag: Option<Position>,

    /// Where the current position of the dragging.
    pub current_drag: Option<Position>,

    /// The current shape to draw.
    pub current_shape: Option<Shape>,

    /// List of existing shapes.
    pub shapes: Vec<Shape>,
}

impl State {
    /// Creates new state.
    pub fn new() -> Self {
        Self {
            dragging: false,
            start_drag: None,
            current_drag: None,
            current_shape: None,
            shapes: Vec::new(),
        }
    }
}
