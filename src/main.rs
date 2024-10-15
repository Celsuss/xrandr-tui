use std::io::{self, Write};

// use ratatui::{
//     crossterm::event::{self, Event, KeyCode, KeyEventKind},
//     widgets::{Block, Paragraph},
// };
use std::process::Command;

mod screen;

fn main() -> std::io::Result<()> {
    // let mut terminal = ratatui::init();
    // loop {
    //     terminal.draw(|frame| {
    //         frame.render_widget(
    //             Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
    //             frame.area(),
    //         );
    //     })?;
    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
    //             break;
    //         }
    //     }
    // }
    // ratatui::restore();

    let mut command = Command::new("sh");
    command.arg("-c").arg("xrandr");
    let output = command.output().expect("failed to execute process");
    println!("Status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    print!("done");
    Ok(())
}
