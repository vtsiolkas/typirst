use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(f: &mut Frame, app: &App) {
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
