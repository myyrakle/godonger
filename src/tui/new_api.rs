use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{io::Result, process::exit};

use crate::{handle, utils::get_config_file_or_warn};

use super::TerminalType;

pub fn run(terminal: &mut TerminalType, domain: String) -> Result<()> {
    terminal.clear()?;

    let mut step = 0;

    let config_file = get_config_file_or_warn();

    let mut api_group = config_file.default_api_group;
    let mut api_path = "/".to_string();
    let mut method_name = String::new();
    let mut has_response = true;

    let mut render_text = String::new();
    loop {
        render_text.clear();

        match step {
            0 => {
                render_text.push_str("Enter API Group: ");
                render_text.push_str(&api_group);
            }
            1 => {
                render_text.push_str("Enter API Path: ");
                render_text.push_str(&api_path);
            }
            2 => {
                render_text.push_str("Enter Method Name: ");
                render_text.push_str(&method_name);
            }
            3 => {
                render_text.push_str("Do you need a response? (y/n): ");
                render_text.push_str(if has_response { "y" } else { "n" });
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
                                api_group.push(c);
                            }
                            1 => {
                                api_path.push(c);
                            }
                            2 => {
                                method_name.push(c);
                            }
                            3 => match c {
                                'y' => {
                                    has_response = true;
                                }
                                'n' => {
                                    has_response = false;
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Backspace => match step {
                            0 => {
                                if !api_group.is_empty() {
                                    api_group.pop();
                                }
                            }
                            1 => {
                                if !api_path.is_empty() {
                                    api_path.pop();
                                }
                            }
                            2 => {
                                if !method_name.is_empty() {
                                    method_name.pop();
                                }
                            }
                            3 => {}
                            _ => {}
                        },
                        KeyCode::Enter => match step {
                            0 | 1 | 2 => {
                                step += 1;
                            }
                            3 => {
                                handle::api::new_api(
                                    domain.clone(),
                                    api_path.clone(),
                                    method_name.clone(),
                                    has_response,
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
