use crate::options::{CyclicOption, Highlight, Labeled};
use crate::text_generator::CharState;
use crate::utils::get_nth_word_boundaries;
use crate::App;
use ratatui::style::palette::tailwind::{EMERALD, RED, SLATE};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    symbols::border,
    text::{Line, Span},
    widgets::{block::Position, block::Title, Block, Borders, Paragraph},
    Frame,
};

struct Colors {
    untyped: Color,
    correct: Color,
    incorrect: Color,
}

fn get_colors(cur_line: usize, line_idx: usize, c: char) -> Colors {
    if line_idx == cur_line && c != ' ' {
        Colors {
            untyped: SLATE.c50,
            correct: EMERALD.c400,
            incorrect: RED.c400,
        }
    } else if (line_idx as isize - cur_line as isize).abs() <= 1 {
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

pub const WIDTH: u16 = 60;

pub fn ui(f: &mut Frame, app: &App) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(4),
        ])
        .split(f.size());

    let title_text = Paragraph::new("Typirst").bold().white().centered();

    f.render_widget(title_text, vertical_layout[0]);

    render_typing_area(f, vertical_layout[2], app);
    render_stats_area(f, vertical_layout[4], app);
    render_message_area(f, vertical_layout[5], app);
    /////////////////////////////////
    // Menu block
    /////////////////////////////////
    if app.pause {
        let menu_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Fill(1),
            ])
            .split(vertical_layout[6]);

        render_options_block(f, menu_block[1], app.difficulty.clone());
        render_options_block(f, menu_block[2], app.highlight.clone());

        let block = Block::default();
        let menu_text = Paragraph::new(vec![
            Line::from(vec![]),
            Line::from(vec![]),
            Line::from(vec![
                Span::from("(q)").style(Style::default().bg(SLATE.c700)),
                Span::from(" quit"),
            ]),
        ])
        .block(block);

        f.render_widget(menu_text, menu_block[3]);

        let block = Block::default();
        let menu_text = Paragraph::new(vec![
            Line::from(vec![]),
            Line::from(vec![]),
            Line::from(vec![
                Span::from("(q)").style(Style::default().bg(SLATE.c700)),
                Span::from(" quit"),
            ]),
        ])
        .block(block);

        f.render_widget(menu_text, menu_block[4]);

        let block = Block::default();
        let menu_text = Paragraph::new(vec![
            Line::from(vec![]),
            Line::from(vec![]),
            Line::from(vec![
                Span::from("(r)").style(Style::default().bg(SLATE.c700)),
                Span::from(" restart"),
            ]),
        ])
        .block(block);

        f.render_widget(menu_text, menu_block[5]);
    }
}

fn render_options_block<T: Labeled>(
    f: &mut Frame,
    layout: Rect,
    option_container: CyclicOption<T>,
) {
    let mut difficulty_options = vec![];
    let options = option_container.surrounding();
    difficulty_options.push(
        Line::from(format!("    {}", options.0.label())).style(Style::default().fg(SLATE.c500)),
    );
    difficulty_options.push(
        Line::from(format!(
            "({}) {}",
            option_container.keybinding,
            options.1.label()
        ))
        .style(Style::default().fg(SLATE.c300)),
    );
    difficulty_options.push(
        Line::from(format!("    {}", options.2.label())).style(Style::default().fg(SLATE.c500)),
    );

    let menu_text = Paragraph::new(difficulty_options);
    f.render_widget(menu_text, layout);
}

fn render_typing_area(f: &mut Frame, layout: Rect, app: &App) {
    let typing_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(WIDTH),
            Constraint::Fill(1),
        ])
        .split(layout);

    let mut typing_lines = vec![];

    let (word_start, word_end, line_offset) =
        get_nth_word_boundaries(app, app.highlight.current().get_words_ahead());

    for line_idx in app.cur_line as isize - 2..app.cur_line as isize + 3 {
        if line_idx < 0 || line_idx >= app.characters.len() as isize {
            typing_lines.push(Line::from(vec![" ".into()]));
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
                        text = text.white().underlined().bold();
                    }
                }
                Highlight::Word => {
                    if (line_idx == app.cur_line as isize) && idx >= word_start && idx < word_end {
                        text = text.yellow().underlined().bold();
                    }
                }
                Highlight::NextWord | Highlight::TwoWords => {
                    if line_offset as isize == line_idx - app.cur_line as isize
                        && idx >= word_start
                        && idx < word_end
                    {
                        text = text.underlined().bold();
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
            + ((WIDTH as f32 - app.characters[app.cur_line].len() as f32) / 2.0).ceil() as u16
            + app.position as u16,
        typing_area[1].y + 2,
    );
}

fn render_stats_area(f: &mut Frame, layout: Rect, app: &App) {
    let stats_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(WIDTH / 2),
            Constraint::Length(WIDTH / 2),
            Constraint::Fill(1),
        ])
        .split(layout);

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

    let elapsed = app.timer.elapsed();
    let wpm: f64 = if elapsed.as_millis() <= 2 {
        0.0
    } else {
        let elapsed = elapsed.as_secs_f64() / 60.0;
        (app.typed_chars as f64 / 5.0) / elapsed
    };

    let wpm_string = if wpm == 0.0 {
        "-".to_string()
    } else {
        format!("{:.1}", wpm)
    };

    let wpm_text = Paragraph::new(wpm_string).centered().block(block);
    f.render_widget(wpm_text, stats_layout[1]);

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

    let accuracy_text = Paragraph::new(format!("{:.0}%", accuracy))
        .centered()
        .block(block);
    f.render_widget(accuracy_text, stats_layout[2]);
}

fn render_message_area(f: &mut Frame, layout: Rect, app: &App) {
    let block = Block::default();
    let message = Paragraph::new(if app.pause {
        vec![
            Line::from(vec![]),
            Line::from(vec![Span::from("PAUSED").bold()]),
            Line::from(vec![Span::from("Press Esc to resume")]),
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
