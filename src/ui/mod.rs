mod common;
mod graph;
mod typing;

use crate::ui::graph::render_graph;
use crate::ui::typing::render_typing;
use crate::App;
use ratatui::widgets::Wrap;
use ratatui::Frame;

pub fn ui(f: &mut Frame, app: &mut App) {
    if app.showing_size_warning {
        render_size_warning(f);
    } else if app.showing_stats {
        render_graph(f, app);
    } else {
        render_typing(f, app);
    }
}

fn render_size_warning(f: &mut Frame) {
    let text = "Please resize the terminal to at least 72x20.";
    f.render_widget(
        ratatui::widgets::Paragraph::new(text).wrap(Wrap { trim: true }),
        f.size(),
    );
}
