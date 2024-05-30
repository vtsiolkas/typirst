use crate::options::Highlight;
use crate::text_generator::CharState;
use crate::ui::common::{
    get_colors, render_accuracy, render_average_wpm, render_cyclic_options_block,
    render_options_keybind_block,
};
use crate::utils::get_nth_word_boundaries;
use crate::{App, TYPING_AREA_WIDTH};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render_typing(f: &mut Frame, app: &mut App) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(5),
        ])
        .split(f.size());

    let title_text = Span::from("Typirst").bold().white().to_centered_line();

    f.render_widget(title_text, vertical_layout[0]);

    render_typing_area(f, vertical_layout[2], app);
    render_stats_area(f, vertical_layout[4], app);
    render_message_area(f, vertical_layout[6], app);
    /////////////////////////////////
    // Menu block
    /////////////////////////////////
    if app.pause {
        let menu_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(13),
                Constraint::Length(16),
                Constraint::Length(17),
                Constraint::Length(13),
                Constraint::Length(12),
                Constraint::Fill(1),
            ])
            .split(vertical_layout[7]);

        render_cyclic_options_block(f, menu_block[1], app.number_of_words.clone());
        render_cyclic_options_block(f, menu_block[2], app.difficulty.clone());
        render_cyclic_options_block(f, menu_block[3], app.highlight.clone());

        render_options_keybind_block(f, menu_block[4], "r", "Restart");
        render_options_keybind_block(f, menu_block[5], "q", "Quit");
    }
}

fn render_typing_area(f: &mut Frame, layout: Rect, app: &mut App) {
    let typing_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(TYPING_AREA_WIDTH),
            Constraint::Fill(1),
        ])
        .split(layout);

    let mut typing_lines = vec![];

    let (word_start, word_end, line_offset) =
        get_nth_word_boundaries(app, app.highlight.current().get_words_ahead());

    for line_idx in app.cur_line as isize - 2..app.cur_line as isize + 3 {
        if line_idx < 0 || line_idx >= app.lines.len() as isize {
            typing_lines.push(Line::from(vec![" ".into()]));
            continue;
        }

        let line = app.lines.get(line_idx as usize).unwrap();
        let mut terminal_line = vec![];
        for (idx, c) in line.iter().enumerate() {
            let mut string = c.typed_c.to_string();
            if c.typed_c == ' ' {
                string = "\u{00B7}".to_string();
            } else if c.typed_c == '\n' {
                string = "¶".to_string();
            }
            let mut text = Span::from(string.clone()).style(match c.state {
                CharState::Untouched => {
                    Style::default().fg(get_colors(app.cur_line, line_idx as usize, c.c).untyped)
                }
                CharState::Correct => {
                    Style::default().fg(get_colors(app.cur_line, line_idx as usize, c.c).correct)
                }
                CharState::Incorrect => {
                    Style::default().fg(get_colors(app.cur_line, line_idx as usize, c.c).incorrect)
                }
            });
            match app.highlight.current() {
                Highlight::Character => {
                    if line_idx == app.cur_line as isize && app.position == idx {
                        text = text.yellow().underlined().bold();
                    }
                }
                Highlight::Word | Highlight::NextWord | Highlight::TwoWords => {
                    if line_offset as isize == line_idx - app.cur_line as isize
                        && idx >= word_start
                        && idx < word_end
                        && string != "\u{00B7}".to_string()
                    {
                        text = text.yellow().underlined().bold();
                    }
                }
                _ => {}
            }

            terminal_line.push(text);
        }
        typing_lines.push(Line::from(terminal_line));
    }

    let typing_text = Paragraph::new(typing_lines).centered();

    f.render_widget(typing_text, typing_area[1]);

    f.set_cursor(
        typing_area[1].x
            + ((TYPING_AREA_WIDTH as f32 - app.lines[app.cur_line].len() as f32) / 2.0).ceil()
                as u16
            + app.position as u16,
        typing_area[1].y + 2,
    );
}

fn render_stats_area(f: &mut Frame, layout: Rect, app: &App) {
    let stats_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(16),
            Constraint::Length(16),
            Constraint::Fill(1),
        ])
        .split(layout);

    render_average_wpm(f, stats_layout[1], app);
    render_accuracy(f, stats_layout[2], app);
}

fn render_message_area(f: &mut Frame, layout: Rect, app: &App) {
    let block = Block::default();
    let message = Paragraph::new(if app.pause {
        vec![
            Line::from(vec![Span::from("PAUSED").white().bold()]),
            Line::from(vec![Span::from("Press Esc to resume")]),
            Line::from(vec![]),
        ]
    } else {
        vec![
            Line::from(vec![]),
            Line::from(vec![]),
            Line::from(vec![
                Span::from("Press Esc to pause").style(Style::default().fg(SLATE.c500))
            ]),
        ]
    })
    .centered()
    .block(block);

    f.render_widget(message, layout);
}
