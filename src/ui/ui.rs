use ratatui::{text::Text, Frame};

use crate::state::State;

/// The struct that will handle all ui rendering stuff.
#[derive(Debug)]
pub struct Ui {}

impl Ui {
    /// Creates new ui.
    pub fn new() -> Self {
        Self {}
    }

    /// Draw the ui.
    pub fn draw(&mut self, frame: &mut Frame<'_>, _state: &mut State) {
        frame.render_widget(
            Text::from("Hello, World"),
            frame.area()
        );
    }
}
