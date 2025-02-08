use std::panic;

use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::event::{EventHandler, TrawEvent};

/// Traw's Result type alias.
pub type TrawResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Main traw application struct.
#[derive(Debug)]
pub struct Traw {
    /// The terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// Whether we should exit.
    pub exit: bool
}

impl Traw {
    /// Creates new traw.
    pub fn new() -> Self {
        // Hook into panics to properly restore the terminal
        // when a panic happened.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            ratatui::restore();
            panic_hook(panic);
        }));

        Self {
            terminal: ratatui::init(),
            exit: false
        }
    }

    /// Starts traw.
    pub async fn run(&mut self) -> TrawResult<()> {
        // Creates a new EventHandler with 30 fps.
        let mut event_handler = EventHandler::new(30);

        // Starts handling events.
        event_handler.handle();

        while !self.exit {
            match event_handler.next().await? {
                TrawEvent::Tick => self.tick(),
                TrawEvent::Keypress(key) => self.keypress(key),
                TrawEvent::Mouse(mouse) => self.mouse(mouse),
                TrawEvent::Resize(width, height) => self.resize(width, height),
            }
        }

        Ok(())
    }

    /// Handles tick event.
    fn tick(&mut self) {}

    /// Handles Keypress event.
    fn keypress(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            // Exit of traw when Esc is pressed.
            crossterm::event::KeyCode::Esc => {
                self.exit();
            }

            _ => {}
        }
    }

    /// Handles mouse event.
    fn mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}

    /// Handles resize event.
    fn resize(&mut self, _width: u16, _height: u16) {}

    /// Exits out of traw.
    pub fn exit(&mut self) {
        ratatui::restore();
        self.exit = true;
    }
}
