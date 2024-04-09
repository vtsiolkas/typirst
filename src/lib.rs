mod splitter;
pub mod tui;
mod utils;

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::palette::tailwind::{EMERALD, RED, SLATE};
use ratatui::style::Color;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::Title, *},
};
use splitter::{split_string, CharState, Character};

#[derive(Debug, Default)]
pub struct App {
    lines: Vec<Vec<Character>>,
    cur_line: usize,
    position: usize,
    exit: bool,
}

impl App {
    fn load_lines(&mut self) -> Result<()> {
        let text = include_str!("../assets/text.txt");
        self.lines = split_string(text, 40);
        Ok(())
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.load_lines().unwrap();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn check_character(&mut self, c: char) {
        self.lines[self.cur_line][self.position].set_typed(c);
        self.position += 1;

        if self.position == self.lines[self.cur_line].len() {
            // Switch to next line or exit if we're at the end
            if self.cur_line == self.lines.len() - 1 {
                self.exit();
            }
            self.position = 0;
            self.cur_line += 1;
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char(c) => {
                self.check_character(c);
            }
            KeyCode::Enter => {
                self.check_character('\n');
            }
            KeyCode::Backspace => {
                // Handle backspace, removing the last character typed

                // Handle if we're at the beginning of the first line
                if self.position == 0 && self.cur_line == 0 {
                    return Ok(());
                }

                if self.position > 0 {
                    self.position -= 1;
                } else {
                    self.cur_line -= 1;
                    self.position = self.lines[self.cur_line].len() - 1;
                }
                self.lines[self.cur_line][self.position].reset();
            }

            KeyCode::Esc => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn get_colors(&self, line_idx: usize) -> Colors {
        if line_idx == self.cur_line {
            Colors {
                untyped: SLATE.c50,
                correct: EMERALD.c400,
                incorrect: RED.c400,
            }
        } else if (line_idx as isize - self.cur_line as isize).abs() <= 1 {
            Colors {
                untyped: SLATE.c400,
                correct: EMERALD.c700,
                incorrect: RED.c800,
            }
        } else {
            Colors {
                untyped: SLATE.c500,
                correct: EMERALD.c800,
                incorrect: RED.c900,
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

struct Colors {
    untyped: Color,
    correct: Color,
    incorrect: Color,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(7),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(52),
                Constraint::Fill(1),
            ])
            .split(outer_layout[1]);
        let title = Title::from(" Typirst ".bold().white());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_style(SLATE.c500)
            .border_set(border::THICK);

        let mut terminal_lines = vec![];
        for line_idx in self.cur_line as isize - 2..self.cur_line as isize + 3 {
            if line_idx < 0 || line_idx >= self.lines.len() as isize {
                terminal_lines.push(Line::from(vec![" ".into()]));
                continue;
            }

            let line = self.lines.get(line_idx as usize).unwrap();
            let mut terminal_line = vec![];
            for (idx, c) in line.iter().enumerate() {
                let mut string = c.typed_c.to_string();
                if c.typed_c == ' ' {
                    string = "\u{00B7}".to_string();
                } else if c.c == '\n' {
                    string = "¶".to_string();
                }
                let mut text = Span::from(string).style(match c.state {
                    CharState::Untouched => {
                        Style::default().fg(self.get_colors(line_idx as usize).untyped)
                    }
                    CharState::Correct => {
                        Style::default().fg(self.get_colors(line_idx as usize).correct)
                    }
                    CharState::Incorrect => {
                        Style::default().fg(self.get_colors(line_idx as usize).incorrect)
                    }
                });
                if line_idx as usize == self.cur_line {
                    if self.position == idx {
                        text = text.bold();
                        text = text.underlined();
                    }
                }
                terminal_line.push(text);
            }
            terminal_lines.push(Line::from(terminal_line));
        }

        let line_text = Text::from(terminal_lines);
        Paragraph::new(line_text)
            .centered()
            .block(block)
            .render(inner_layout[1], buf);
    }
}
