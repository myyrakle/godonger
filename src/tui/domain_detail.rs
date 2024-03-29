use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::io::Result;

use crate::{
    handle::{
        lookup::lookup_internal,
        render::{render_domain_files, render_internal_files},
    },
    utils::get_scroll_point,
};

use super::TerminalType;

pub fn run(terminal: &mut TerminalType, domain: String) -> Result<()> {
    terminal.clear()?;

    let mut selected_index = 0;

    let items = vec![
        "1. Add New API (with usecase)",
        "2. Add Method To Usecase",
        "3. Add Method To Store",
        "4. Add Method To Helper",
        "5. Add New Usecase",
        "6. Add New Store",
        "7. Add New Helper",
    ];

    let domain_files = crate::handle::lookup::lookup_domain(domain.clone());
    let domain_files_text = render_domain_files(&domain_files);

    let internal_files = lookup_internal(domain.clone());
    let internal_files_text = render_internal_files(&internal_files);

    let mut render_text = String::new();
    loop {
        render_text.clear();
        for (i, domain) in items.iter().enumerate() {
            if i == selected_index {
                render_text.push_str(&format!("▶ {}\n", domain));
            } else {
                render_text.push_str(&format!("  {}\n", domain));
            }
        }

        render_text.push_str(&format!("\n{}", domain_files_text));
        render_text.push_str(&format!("\n{}", internal_files_text));

        let block = Block::default()
            .title(format!("Domain: {}", domain))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(render_text.clone())
            .block(block)
            .scroll(get_scroll_point(selected_index));

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
                            if selected_index < items.len() - 1 {
                                selected_index += 1;
                            }
                        }
                        KeyCode::Up => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Enter => match selected_index {
                            0 => {
                                crate::tui::new_api::run(terminal, domain.clone())?;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
