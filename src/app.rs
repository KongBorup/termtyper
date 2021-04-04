use crate::typer;
use crossterm::{
    cursor::{self, MoveTo, MoveToNextLine},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{self, Color, Colors, SetColors, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use std::io::Write;

const INFO: &str = "To exit, press ESC";

pub fn run<W: Write>(output: &mut W, input: String) -> crossterm::Result<()> {
    let mut typer = typer::Typer::new(input);

    enable_raw_mode()?;

    loop {
        queue!(
            output,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0),
        )?;

        queue!(
            output,
            SetForegroundColor(Color::Green),
            style::Print(typer.get_correct()),
            SetColors(Colors::new(Color::Black, Color::Red)),
            style::Print(typer.get_incorrect()),
            style::ResetColor,
            SetForegroundColor(Color::White),
            style::Print(typer.get_missing()),
            MoveToNextLine(2),
            SetForegroundColor(Color::White),
            style::Print(typer.get_typed_chars()),
            style::Print("_"),
            MoveToNextLine(2),
        )?;

        if typer.is_finished() {
            queue!(
                output,
                SetForegroundColor(Color::Green),
                style::Print("FINISHED!"),
                MoveToNextLine(2),
            )?;
        }

        queue!(
            output,
            SetForegroundColor(Color::White),
            style::Print(format!("{:.0} wpm", typer.wpm().round())),
            MoveToNextLine(1),
            style::Print(format!("{:.1}% acc", typer.accuracy() * 100.0)),
            MoveToNextLine(1),
            style::Print(typer.elapsed_time_string()),
            MoveToNextLine(2),
        )?;

        queue!(
            output,
            SetForegroundColor(Color::White),
            style::Print(INFO),
            MoveToNextLine(1)
        )?;

        output.flush()?;

        let inp = read_input()?;

        if !typer.is_finished() {
            match inp {
                KeyCode::Char(c) => typer.write(c),
                KeyCode::Backspace => typer.remove_last(),
                _ => {}
            };
        }

        if let KeyCode::Esc = inp {
            break;
        }
    }

    execute!(
        output,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        MoveTo(0, 0),
    )?;
    disable_raw_mode()?;

    Ok(())
}

pub fn read_input() -> crossterm::Result<KeyCode> {
    loop {
        if let Event::Key(k) = event::read()? {
            return Ok(k.code);
        }
    }
}
