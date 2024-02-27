use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{io::Result, process::exit};

use crate::handle;

use super::TerminalType;

pub fn run(terminal: &mut TerminalType, domain: String) -> Result<()> {
    terminal.clear()?;

    let mut step = 0;

    let mut api_path = "/".to_string();
    let mut method_name = String::new();

    let mut render_text = String::new();
    loop {
        render_text.clear();

        match step {
            0 => {
                render_text.push_str("Enter API Path: ");
                render_text.push_str(&api_path);
            }
            1 => {
                render_text.push_str("Enter Method Name: ");
                render_text.push_str(&method_name);
            }
            _ => {}
        }

        let block = Block::default()
            .title(format!("New API: {}", domain))
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
                        KeyCode::Char(c) => match step {
                            0 => {
                                api_path.push(c);
                            }
                            1 => {
                                method_name.push(c);
                            }
                            _ => {}
                        },
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Backspace => match step {
                            0 => {
                                if !api_path.is_empty() {
                                    api_path.pop();
                                }
                            }
                            1 => {
                                if !method_name.is_empty() {
                                    method_name.pop();
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Enter => match step {
                            0 => {
                                step = 1;
                            }
                            1 => {
                                handle::api::new_api(
                                    domain.clone(),
                                    api_path.clone(),
                                    method_name.clone(),
                                );
                                exit(0);
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
