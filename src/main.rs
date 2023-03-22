use std::{io, error::Error, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, ListItem, List},
    layout::{Layout, Constraint, Direction},
    Terminal, style::{Modifier, Style, Color}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ureq;
use feed_rs::{self, parser::parse};

fn main() -> Result<(), Box<dyn Error>> {
    let raw_feed: String = ureq::get("https://www.youtube.com/feeds/videos.xml?channel_id=UCUyeluBRhGPCW4rPe_UvBZQ")
            .call()?
            .into_string()?;

    let feed = parse(raw_feed.as_bytes())?;
    let mut list = vec![];
    
    for item in feed.entries {
        list.push(ListItem::new(item.title.expect("where is the title?").content));
    }

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(feed.title.expect("there should be a title for the channel").content)
            .borders(Borders::ALL);
        let list_widget = List::new(list)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        f.render_widget(list_widget, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
