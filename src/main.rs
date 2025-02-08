use traw::{Traw, TrawResult};

mod traw;
mod event;
mod ui;

#[tokio::main]
async fn main() -> TrawResult<()> {
    // Starts traw
    Traw::new()
        .run()
        .await?;

    Ok(())
}
