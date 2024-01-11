use std::io;
use std::time::Duration;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

fn main() -> io::Result<()> {
    io::stdout().execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let mut app = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    app.clear()?;

    loop {
        app.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Heddo! ([q] to quit)").white().on_blue(),
                area,
            );
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    io::stdout().execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
