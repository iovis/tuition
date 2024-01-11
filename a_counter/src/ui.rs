use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "
              Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
              Press [j] and [k] to control the counter.\n\
              Counter: {}
            ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        frame.size(),
    );
}
