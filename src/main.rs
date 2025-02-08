use traw::{Traw, TrawResult};

mod traw;
mod event;

#[tokio::main]
async fn main() -> TrawResult<()> {
    // Starts traw
    Traw::new()
        .run()
        .await?;

    Ok(())
}
