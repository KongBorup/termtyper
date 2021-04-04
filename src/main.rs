use std::io::stdout;

mod app;
mod typer;

const TEXT: &str = "Life is not suffering; it's just that you will suffer it, rather than enjoy it, until you let go of your mind's attachments and just go for the ride freely, no matter what happens.";

fn main() -> crossterm::Result<()> {
    let output = &mut stdout();
    app::run(output, String::from(TEXT))?;
    Ok(())
}
