use std::io::stdout;

mod app;
mod typer;

const TEXT: &str = "It is easy to kill with a bow, girl. How easy it is to release the bowstring and think, it is not I, it is the arrow. The blood of that boy is not on my hands. The arrow killed him, not I. But the arrow does not dream anything in the night.";

fn main() -> crossterm::Result<()> {
    let output = &mut stdout();
    app::run(output, String::from(TEXT))?;
    Ok(())
}
