use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::{Frame, Terminal};

struct App {
    counter: i64,
    should_quit: bool,
}

fn main() -> Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;

    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    while !app.should_quit {
        terminal.draw(|frame| ui(&app, frame))?;
        update(&mut app)?;
    }

    Ok(())
}

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter)),
        frame.size(),
    );
}

fn update(app: &mut App) -> Result<()> {
    if crossterm::event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = crossterm::event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Char('k') => app.counter += 1,
                    KeyCode::Char('j') => app.counter -= 1,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
