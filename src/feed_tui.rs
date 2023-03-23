pub mod tui {
    use std::{io, error::Error};
    use tui::{
        backend::CrosstermBackend,
        widgets::{
            Block, 
            Borders, 
            ListItem, 
            List
        },
        Terminal, style::{Modifier, Style, Color}
    };
    use crossterm::{
        event::{
            DisableMouseCapture,
        },
        execute,
        terminal::{
            LeaveAlternateScreen
        },
    };
    use ureq;
    use feed_rs::{self, parser::parse};
    
    fn get_feed(url: &str) -> Result<String, Box<dyn Error>> {
        let raw_feed: String = ureq::get(url)
        .call()?
        .into_string()?;
        Ok(raw_feed)
    }

    macro_rules! draw {
        ($terminal:expr, $list:expr) => {
            $terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .title("YouTube")
                    .borders(Borders::ALL);
                let list_widget = List::new($list)
                .block(block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
            
                f.render_widget(list_widget, size);
            })?;
        }
    }

    pub fn render() -> Result<(), Box<dyn Error>> {
        let raw_feed = get_feed(r#"https://www.youtube.com/feeds/videos.xml?channel_id=UCUyeluBRhGPCW4rPe_UvBZQ"#)?;
        let feed = parse(raw_feed.as_bytes())?;
        let mut list = vec![];
        
        for item in feed.entries {
            list.push(ListItem::new(item.title.expect("where is the title?").content));
        }
            
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        draw!(terminal, list);
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        
        terminal.show_cursor()?;
        Ok(())
    }
}