use traw::{Traw, TrawResult};

mod event;
mod shape;
mod state;
mod traw;
mod ui;

#[tokio::main]
async fn main() -> TrawResult<()> {
    // Starts traw
    Traw::new()?.run().await?;

    Ok(())
}
