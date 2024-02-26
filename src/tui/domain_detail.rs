use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{io::Result, path::PathBuf};

use crate::utils::get_config_file_or_warn;

use super::TerminalType;

pub fn run(terminal: &mut TerminalType, domain: String) -> Result<()> {
    terminal.clear()?;

    let mut selected_index = 0;

    let items = vec![
        "1. Add New API (with usecase)",
        "2. Add New Usecase",
        "3. Add New Store",
        "4. Add New Helper",
    ];

    let domain_files: Vec<String> = crate::handle::lookup::lookup_domain(domain.clone())
        .into_iter()
        .map(|path| {
            let path: PathBuf = path.components().skip(2).collect();
            path.to_str().unwrap().to_string()
        })
        .collect();

    let config_file = get_config_file_or_warn();

    let mut domain_files_text = String::new();
    domain_files_text.push_str(&format!("{}\n", config_file.domain_dir.to_str().unwrap()));
    for domain_file in domain_files {
        domain_files_text.push_str(&format!("|-- {}\n", domain_file));
    }

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

        render_text.push_str(&format!("\n\n{}", domain_files_text));

        let block = Block::default()
            .title(format!("Domain: {}", domain))
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
                        KeyCode::Enter => {}
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
