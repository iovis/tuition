use anyhow::Result;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use tuition::app::App;
use tuition::event::{Event, EventHandler};
use tuition::tui::Tui;
use tuition::update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}
