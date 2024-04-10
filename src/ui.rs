use ratatui::layout::Offset;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::widgets::block::Position;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

use crate::text_generator::CharState;
use crate::App;

pub fn ui(f: &mut Frame, app: &App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(15),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(f.size());

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
        .borders(Borders::NONE)
        .border_style(SLATE.c500)
        .border_set(border::THICK);

    let mut terminal_lines = vec![];
    terminal_lines.push(Line::from(vec![" ".into()]));
    terminal_lines.push(Line::from(vec![" ".into()]));
    terminal_lines.push(Line::from(vec![" ".into()]));
    terminal_lines.push(Line::from(vec![" ".into()]));

    for line_idx in app.cur_line as isize - 2..app.cur_line as isize + 3 {
        if line_idx < 0 || line_idx >= app.characters.len() as isize {
            terminal_lines.push(Line::from(vec![" ".into()]));
            continue;
        }

        let line = app.characters.get(line_idx as usize).unwrap();
        let mut terminal_line = vec![];
        for (idx, c) in line.iter().enumerate() {
            let mut string = c.typed_c.to_string();
            if c.typed_c == ' ' {
                string = "\u{00B7}".to_string();
            } else if c.typed_c == '\n' {
                string = "¶".to_string();
            }
            let mut text = Span::from(string).style(match c.state {
                CharState::Untouched => {
                    Style::default().fg(app.get_colors(line_idx as usize).untyped)
                }
                CharState::Correct => {
                    Style::default().fg(app.get_colors(line_idx as usize).correct)
                }
                CharState::Incorrect => {
                    Style::default().fg(app.get_colors(line_idx as usize).incorrect)
                }
            });
            if line_idx as usize == app.cur_line {
                if app.position == idx {
                    text = text.bold();
                    text = text.underlined();
                }
            }
            terminal_line.push(text);
        }
        terminal_lines.push(Line::from(terminal_line));
    }

    let line_text = Text::from(terminal_lines);
    let typing_text = Paragraph::new(line_text).centered().block(block);

    f.render_widget(typing_text, inner_layout[1]);

    // WPM and accuracy
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(26),
            Constraint::Length(26),
            Constraint::Fill(1),
        ])
        .split(outer_layout[2].offset(Offset { x: 0, y: -1 }));

    let title = Title::from(" WPM ".white());
    let block = Block::default()
        .title(
            title
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_style(SLATE.c500)
        .border_set(border::THICK);
    let elapsed = app.start_time.elapsed().as_secs_f64() / 60.0;
    let wpm: f64 = (app.typed_chars as f64 / 5.0) / elapsed;

    let wpm_block = Paragraph::new(format!("{:.1}", wpm))
        .centered()
        .block(block);
    f.render_widget(wpm_block, inner_layout[1]);

    let title = Title::from(" Accuracy ".white());
    let block = Block::default()
        .title(
            title
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_style(SLATE.c500)
        .border_set(border::THICK);

    let accuracy = if app.typed_chars > 0 {
        (1.0 - app.errors as f64 / app.typed_chars as f64) * 100.0
    } else {
        100.0
    };

    let errors_block = Paragraph::new(format!("{:.0}%", accuracy))
        .centered()
        .block(block);
    f.render_widget(errors_block, inner_layout[2]);
}
