use ratatui::layout::Position;

use crate::state::State;

use super::shape::Shape;

#[derive(PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

/// Helper function for drawing a line.
fn draw_line(
    buf: &mut ratatui::prelude::Buffer,
    start: &Position,
    end: &Position,
    direction: Direction,
) {
    match direction {
        Direction::Horizontal => {
            if start.x < end.x {
                for point in (start.x + 1)..=(end.x - 1) {
                    buf[(point, start.y)].set_char('━');
                }
            } else {
                for point in (end.x + 1)..=(start.x - 1) {
                    buf[(point, start.y)].set_char('━');
                }
            }
        }
        Direction::Vertical => {
            if start.y < end.y {
                for point in (start.y + 1)..=(end.y - 1) {
                    buf[(start.x, point)].set_char('┃');
                }
            } else {
                for point in (end.y + 1)..=(start.y - 1) {
                    buf[(start.x, point)].set_char('┃');
                }
            }
        }
    }
}

/// Render rectangle shape.
pub fn render(
    shape: &Shape,
    _area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
    _state: &mut State,
) {
    if let Shape::Rectangle { p1, p2, p3, p4 } = shape {
        // Draw top line.
        draw_line(buf, p1, p2, Direction::Horizontal);

        // Draw bottom line.
        draw_line(buf, p4, p3, Direction::Horizontal);

        // Draw right line.
        draw_line(buf, p2, p3, Direction::Vertical);

        // Draw left line.
        draw_line(buf, p1, p4, Direction::Vertical);
    }
}
