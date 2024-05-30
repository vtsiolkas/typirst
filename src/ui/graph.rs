use crate::ui::common::{
    render_accuracy, render_average_wpm, render_errors, render_options_keybind_block,
};
use crate::utils::calculate_wpm_and_errors_datasets;
use crate::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::{block::Title, Axis, Block, Chart, Dataset, GraphType},
    Frame,
};

pub fn render_graph(f: &mut Frame, app: &mut App) {
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
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
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
