use crate::options::{CyclicOption, Labeled};
use crate::App;
use ratatui::style::palette::tailwind::{EMERALD, RED, SLATE};
use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Colors {
    pub untyped: Color,
    pub correct: Color,
    pub incorrect: Color,
}

pub fn get_colors(cur_line: usize, line_idx: usize, c: char) -> Colors {
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

pub fn render_options_keybind_block(f: &mut Frame, layout: Rect, keybinding: &str, label: &str) {
    let block = Block::default();
    let menu_text = Paragraph::new(vec![Line::from(format!(" {} ({}) ", label, keybinding))
        .centered()
        .style(Style::default().bold().bg(SLATE.c800).fg(SLATE.c100))])
    .block(block);

    f.render_widget(menu_text, layout);
}

pub fn render_cyclic_options_block<T: Labeled>(
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

pub fn render_stats_block(f: &mut Frame, layout: Rect, title: &str, value: String) {
    let title = Span::from(title)
        .style(Style::default().fg(SLATE.c500))
        .to_centered_line();
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(SLATE.c500)
        .border_set(border::THICK);

    let text = Paragraph::new(value).white().centered().block(block);
    f.render_widget(text, layout);
}

pub fn render_average_wpm(f: &mut Frame, layout: Rect, app: &App) {
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

pub fn render_accuracy(f: &mut Frame, layout: Rect, app: &App) {
    let accuracy = if app.typed_chars > 0 {
        (app.typed_chars as f64 - app.errors as f64) / app.typed_chars as f64 * 100.0
    } else {
        100.0
    };

    render_stats_block(f, layout, " Accuracy ", format!("{:.0}%", accuracy));
}

pub fn render_errors(f: &mut Frame, layout: Rect, app: &App) {
    render_stats_block(f, layout, " Errors ", format!("{:.0}", app.errors));
}
