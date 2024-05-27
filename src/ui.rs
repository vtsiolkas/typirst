use crate::options::{CyclicOption, Highlight, Labeled};
use crate::text_generator::CharState;
use crate::utils::{calculate_wpm_and_errors_datasets, get_nth_word_boundaries};
use crate::App;
use ratatui::style::palette::tailwind::{EMERALD, RED, SLATE};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    symbols::border,
    text::{Line, Span},
    widgets::{block::Title, Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
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

pub fn ui(f: &mut Frame, app: &mut App) {
    if app.showing_stats {
        render_stats(f, app);
    } else {
        render_typing(f, app);
    }
}

fn render_typing(f: &mut Frame, app: &mut App) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(5),
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
                Constraint::Length(13),
                Constraint::Length(18),
                Constraint::Length(22),
                Constraint::Length(18),
                Constraint::Length(18),
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

fn render_options_keybind_block(f: &mut Frame, layout: Rect, keybinding: &str, label: &str) {
    let block = Block::default();
    let menu_text = Paragraph::new(vec![Line::from(format!(" {} ({}) ", label, keybinding))
        .centered()
        .style(Style::default().bold().bg(SLATE.c800).fg(SLATE.c100))])
    .block(block);

    f.render_widget(menu_text, layout);
}

fn render_cyclic_options_block<T: Labeled>(
    f: &mut Frame,
    layout: Rect,
    option_container: CyclicOption<T>,
) {
    let mut visible_options = vec![];
    let options = option_container.surrounding();
    visible_options.push(
        Line::from(format!(
            " {} ({}) ",
            option_container.label, option_container.keybinding
        ))
        .centered()
        .style(Style::default().bold().bg(SLATE.c800).fg(SLATE.c100)),
    );
    visible_options.push(
        Line::from(format!("{}", options.0.label()))
            .centered()
            .style(Style::default().fg(SLATE.c500)),
    );
    visible_options.push(
        Line::from(format!("{}", options.1.label()))
            .centered()
            .style(Style::default().bold().fg(SLATE.c300)),
    );
    visible_options.push(
        Line::from(format!("{}", options.2.label()))
            .centered()
            .style(Style::default().fg(SLATE.c500)),
    );

    let menu_text = Paragraph::new(visible_options);
    f.render_widget(menu_text, layout);
}

fn render_typing_area(f: &mut Frame, layout: Rect, app: &mut App) {
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
            + ((WIDTH as f32 - app.lines[app.cur_line].len() as f32) / 2.0).ceil() as u16
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

    render_average_wpm(f, stats_layout[1], app);
    render_accuracy(f, stats_layout[2], app);
}

fn render_stats_block(f: &mut Frame, layout: Rect, title: &str, value: String) {
    let block = Block::default()
        .title(title.white())
        .borders(Borders::ALL)
        .border_style(SLATE.c500)
        .border_set(border::THICK);

    let text = Paragraph::new(value).centered().block(block);
    f.render_widget(text, layout);
}

fn render_average_wpm(f: &mut Frame, layout: Rect, app: &App) {
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
        format!("{:.0}", wpm)
    };

    render_stats_block(f, layout, " WPM ", wpm_string);
}

fn render_accuracy(f: &mut Frame, layout: Rect, app: &App) {
    let accuracy = if app.typed_chars > 0 {
        (app.typed_chars as f64 - app.errors as f64) / app.typed_chars as f64 * 100.0
    } else {
        100.0
    };

    render_stats_block(f, layout, " Accuracy ", format!("{:.0}%", accuracy));
}

fn render_errors(f: &mut Frame, layout: Rect, app: &App) {
    render_stats_block(f, layout, " Errors ", format!("{:.0}", app.errors));
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

fn render_stats(f: &mut Frame, app: &mut App) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(4),
        ])
        .split(f.size());

    let graph_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(vertical_layout[1]);

    let (wpm_data, error_data) = calculate_wpm_and_errors_datasets(&app.stats);
    let datasets = vec![
        // Line chart
        Dataset::default()
            .name("WPM")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().yellow())
            .data(&wpm_data),
        // Scatter chart
        Dataset::default()
            .name("Errors")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().red())
            .data(&error_data),
    ];

    let max_secs = app.stats.last().unwrap().duration_since_start.as_secs_f64();
    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("Time".green())
        .style(Style::default().white())
        .bounds([0.0, max_secs])
        .labels(vec!["0".into(), format!("{:.2}", max_secs).into()]);

    let max_wpm = wpm_data
        .iter()
        .map(|&(_, wpm)| wpm)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("WPM".green())
        .style(Style::default().white())
        .bounds([0.0, max_wpm + 10.0])
        .labels(vec![
            "0".into(),
            format!("{:.0}", max_wpm / 2.0).into(),
            format!("{:.0}", max_wpm).into(),
        ]);

    // Create the chart and link all the parts together
    let title = Title::from("WPM chart".white().bold());
    let chart = Chart::new(datasets)
        .block(Block::new().title(title.alignment(Alignment::Center)))
        .x_axis(x_axis)
        .y_axis(y_axis);
    f.render_widget(chart, graph_layout[1]);

    // Stats layout
    let stats_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(2),
            Constraint::Length(18),
            Constraint::Fill(1),
            Constraint::Length(18),
            Constraint::Fill(1),
            Constraint::Length(18),
            Constraint::Fill(2),
        ])
        .split(vertical_layout[2]);
    render_average_wpm(f, stats_layout[1], app);
    render_errors(f, stats_layout[3], app);
    render_accuracy(f, stats_layout[5], app);

    // Options layout
    let options_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(18),
            Constraint::Length(18),
            Constraint::Fill(1),
        ])
        .split(vertical_layout[4]);
    render_options_keybind_block(f, options_layout[1], "r", "Restart");
    render_options_keybind_block(f, options_layout[2], "q", "Quit");
}
