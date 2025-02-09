use ratatui::{
    layout::Position, Frame
};

use crate::{shape::shape::Shape, state::State};

/// The struct that will handle all ui rendering stuff.
#[derive(Debug)]
pub struct Ui {}

impl Ui {
    /// Creates new ui.
    pub fn new() -> Self {
        Self {}
    }

    /// Draw the ui.
    pub fn draw(&mut self, frame: &mut Frame<'_>, state: &mut State) {
        // Render the current interacting shape.
        if let Some(current_shape) = state.current_shape.clone() {
            frame.render_stateful_widget(
                &current_shape,
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
    pub fn mouse_drag(&mut self, mouse: crossterm::event::MouseEvent, state: &mut State) {
        // Checks to see if we currently dragging and if not set the
        // self.dragging & self.start_drag to say we are.
        if !state.dragging && state.start_drag.is_none() {
            state.dragging = true;
            state.start_drag = Some(Position::new(mouse.column, mouse.row));
        }

        // Checks if we are currently dragging.
        if state.dragging {
            state.current_drag = Some(Position::new(mouse.column, mouse.row));

            if let Some(start_drag) = &state.start_drag {
                if let Some(current_drag) = &state.current_drag {
                    state.current_shape = Some(Shape::Rectangle {
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
        state.dragging = false;
        state.start_drag = None;
        state.current_drag = None;

        if let Some(current_shape) = state.current_shape.clone() {
            state.shapes.push(current_shape);
        }
    }
}
