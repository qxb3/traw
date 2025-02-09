use ratatui::{
    layout::Position, Frame
};

use crate::{shape::shape::Shape, state::State};

/// The struct that will handle all ui rendering stuff.
#[derive(Debug)]
pub struct Ui {
    /// Whether the user is currently dragging.
    dragging: bool,

    /// Where the start position of the dragging starts.
    start_drag: Option<Position>,

    /// Where the current position of the dragging.
    current_drag: Option<Position>,

    /// The current shape to draw.
    current_shape: Option<Shape>,
}

impl Ui {
    /// Creates new ui.
    pub fn new() -> Self {
        Self {
            dragging: false,
            start_drag: None,
            current_drag: None,
            current_shape: None
        }
    }

    /// Draw the ui.
    pub fn draw(&mut self, frame: &mut Frame<'_>, state: &mut State) {
        // Render the current interacting shape.
        if let Some(current_shape) = &self.current_shape {
            frame.render_stateful_widget(
                current_shape,
                frame.area(),
                state
            );
        }

        // Render the existing shapes.
        for shape in state.shapes.iter().cloned().collect::<Vec<Shape>>() {
            frame.render_stateful_widget(
                &shape,
                frame.area(),
                state
            );
        }
    }

    /// Handles mouse click.
    pub fn mouse_click(&mut self, _mouse: crossterm::event::MouseEvent) {}

    /// Handles mouse drag.
    pub fn mouse_drag(&mut self, mouse: crossterm::event::MouseEvent) {
        // Checks to see if we currently dragging and if not set the
        // self.dragging & self.start_drag to say we are.
        if !self.dragging && self.start_drag.is_none() {
            self.dragging = true;
            self.start_drag = Some(Position::new(mouse.column, mouse.row));
        }

        // Checks if we are currently dragging.
        if self.dragging {
            self.current_drag = Some(Position::new(mouse.column, mouse.row));

            if let Some(start_drag) = &self.start_drag {
                if let Some(current_drag) = &self.current_drag {
                    self.current_shape = Some(Shape::Rectangle {
                        p1: *start_drag,
                        p2: Position::new(current_drag.x, start_drag.y),
                        p3: Position::new(current_drag.x, current_drag.y),
                        p4: Position::new(start_drag.x, current_drag.y),
                    })
                }
            }
        }
    }

    /// Handles mouse release.
    pub fn mouse_release(&mut self, _mouse: crossterm::event::MouseEvent, state: &mut State) {
        // Resets dragging states.
        self.dragging = false;
        self.start_drag = None;
        self.current_drag = None;

        if let Some(current_shape) = &self.current_shape {
            state.shapes.push(current_shape.clone());
        }
    }
}
