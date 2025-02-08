use std::time::Duration;

use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::traw::TrawResult;

/// List of events that will be used troughout traw.
#[derive(Debug)]
pub enum TrawEvent {
    /// Terminal render / update.
    Tick,

    /// Keyboard press.
    Keypress(crossterm::event::KeyEvent),

    /// Mouse events.
    Mouse(crossterm::event::MouseEvent),

    /// Terminal resize.
    Resize(u16, u16),
}

/// The struct that will handle events.
#[derive(Debug)]
pub struct EventHandler {
    /// Event send channel.
    sender: mpsc::UnboundedSender<TrawEvent>,

    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<TrawEvent>,

    /// Tick event rate.
    tick_rate: Duration,
}

impl EventHandler {
    /// Creates new EventHandler.
    pub fn new(fps: u64) -> EventHandler {
        let (sender, receiver) = mpsc::unbounded_channel();
        let tick_rate = Duration::from_millis(1000 / fps);

        Self {
            sender,
            receiver,
            tick_rate,
        }
    }

    /// Starts a seperate tokio thread and listen for terminal events
    /// And sends those event back.
    pub fn handle(&self) {
        let sender = self.sender.clone();
        let tick_rate = self.tick_rate.clone();

        tokio::spawn(async move {
            let mut event_stream = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let tick = tick_interval.tick();
                let crossterm_event = event_stream.next().fuse();

                tokio::select! {
                    // If the event channel has been closed, exit out of this loop.
                    _ = sender.closed() => {
                        break;
                    }

                    // Tick event.
                    _ = tick => {
                        sender.send(TrawEvent::Tick).unwrap();
                    }

                    Some(Ok(event)) = crossterm_event => {
                        match event {
                            // Keyboard presses.
                            CrosstermEvent::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => {
                                sender.send(TrawEvent::Keypress(key)).unwrap();
                            }

                            // Mouse events.
                            CrosstermEvent::Mouse(mouse) => {
                                sender.send(TrawEvent::Mouse(mouse)).unwrap();
                            }

                            // Terminal resize.
                            CrosstermEvent::Resize(width, height) => {
                                sender.send(TrawEvent::Resize(width, height)).unwrap();
                            }

                            // Ignore the rest of terminal event.
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    /// Receive the next event from the handle thread.
    pub async fn next(&mut self) -> TrawResult<TrawEvent> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to receive an event from EventHandler.",
            )))
    }
}
