use ratatui::{layout::Position, widgets::StatefulWidget};

use crate::state::State;

use super::rectangle;

/// List of shape.
#[derive(Debug, Clone)]
pub enum Shape {
    /// Rectangle shape.
    Rectangle {
        /// Top left point of the rectangle.
        p1: Position,

        /// Top right point of the rectangle.
        p2: Position,

        /// Bottom right point of the rectangle.
        p3: Position,

        /// Bottom left point of the rectangle.
        p4: Position,
    },

    /// Line shape.
    Line {
        /// Starting point of the line.
        p1: Position,

        /// Middle point of the line.
        p2: Position,

        /// Ending point of the line.
        p3: Position,
    },
}

impl StatefulWidget for &Shape {
    type State = State;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        // Render their corresponding shape.
        match self {
            Shape::Rectangle { .. } => rectangle::render(&self, area, buf, state),
            Shape::Line { .. } => todo!(),
        }
    }
}
