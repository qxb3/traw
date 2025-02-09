use ratatui::Frame;

use crate::{shape::shape::Shape, state::State};

/// Renders the ui.
pub fn render(frame: &mut Frame<'_>, state: &mut State) {
    // Render the current interacting shape.
    if let Some(current_shape) = state.current_shape.clone() {
        frame.render_stateful_widget(&current_shape, frame.area(), state);
    }

    // Render the existing shapes.
    for shape in state.shapes.iter().cloned().collect::<Vec<Shape>>() {
        frame.render_stateful_widget(&shape, frame.area(), state);
    }
}
