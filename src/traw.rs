use std::panic;

use crossterm::{event::EnableMouseCapture, execute};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{
    event::{EventHandler, TrawEvent},
    state::State,
    ui::Ui,
};

/// Traw's Result type alias.
pub type TrawResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Main traw application struct.
#[derive(Debug)]
pub struct Traw {
    /// The terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// The event handler.
    event_handler: EventHandler,

    /// The ui.
    ui: Ui,

    /// The state.
    state: State,

    /// Whether we should exit.
    pub exit: bool,
}

impl Traw {
    /// Creates new traw.
    pub fn new() -> TrawResult<Self> {
        // Hook into panics to properly restore the terminal
        // when a panic happened.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            ratatui::restore();
            panic_hook(panic);
        }));

        // Enables mouse capture.
        execute!(std::io::stdout(), EnableMouseCapture)?;

        Ok(Self {
            terminal: ratatui::init(),
            event_handler: EventHandler::new(60),
            ui: Ui::new(),
            state: State::new(),
            exit: false,
        })
    }

    /// Starts traw.
    pub async fn run(&mut self) -> TrawResult<()> {
        // Starts handling events.
        self.event_handler.handle();

        // Continuesly read incoming events and
        // handle them while we are not exiting.
        while !self.exit {
            match self.event_handler.next().await? {
                TrawEvent::Tick => self.tick()?,
                TrawEvent::Keypress(key) => self.keypress(key),
                TrawEvent::Mouse(mouse) => self.mouse(mouse),
                TrawEvent::Resize(width, height) => self.resize(width, height),
            }
        }

        Ok(())
    }

    /// Handles tick event.
    fn tick(&mut self) -> TrawResult<()> {
        // Draw ui.
        self.terminal.draw(|frame| {
            self.ui.draw(frame, &mut self.state);
        })?;

        Ok(())
    }

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
    fn mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        match mouse.kind {
            // Left click.
            crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                self.ui.mouse_click(mouse);
            }

            // Left mouse drag.
            crossterm::event::MouseEventKind::Drag(crossterm::event::MouseButton::Left) => {
                self.ui.mouse_drag(mouse, &mut self.state);
            }

            // Mouse release.
            crossterm::event::MouseEventKind::Up(crossterm::event::MouseButton::Left) => {
                self.ui.mouse_release(mouse, &mut self.state);
            }

            _ => {}
        }
    }

    /// Handles resize event.
    fn resize(&mut self, _width: u16, _height: u16) {}

    /// Exits out of traw.
    pub fn exit(&mut self) {
        ratatui::restore();
        self.exit = true;
    }
}
