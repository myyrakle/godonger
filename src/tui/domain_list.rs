use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::io::Result;

use super::TerminalType;

pub fn run(terminal: &mut TerminalType, domain_list: Vec<String>) -> Result<()> {
    // let domain_list: Vec<String> = vec![
    //     "foo", "bar", "baz", "a", "b", "c", "d", "e", "fff", "asdf", "asdf",
    // ]
    // .into_iter()
    // .map(|s| s.to_string())
    // .collect();

    if domain_list.is_empty() {
        println!("No domain found");
        return Ok(());
    }

    terminal.clear()?;

    let mut selected_index = 0;

    let mut render_text = String::new();
    loop {
        render_text.clear();
        for (i, domain) in domain_list.iter().enumerate() {
            if i == selected_index {
                render_text.push_str(&format!("▶ {}\n", domain));
            } else {
                render_text.push_str(&format!("  {}\n", domain));
            }
        }

        let block = Block::default()
            .title("Domain List")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(render_text.clone()).block(block);

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(paragraph, area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Down => {
                            if selected_index < domain_list.len() - 1 {
                                selected_index += 1;
                            }
                        }
                        KeyCode::Up => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            super::domain_detail::run(
                                terminal,
                                domain_list[selected_index].clone(),
                            )?;

                            continue;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
