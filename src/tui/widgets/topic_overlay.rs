//! Topic filter overlay rendering.
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::tui::widgets::filter_state::{TopicFilterState, TopicInputMode};

/// Renders the topic filter overlay on top of the current frame.
pub fn render_topic_overlay(
    frame: &mut Frame,
    topic_filter: &mut TopicFilterState,
    filtered_count: usize,
) {
    let overlay_area = frame
        .area()
        .centered(Constraint::Percentage(70), Constraint::Percentage(80));

    frame.render_widget(Clear, overlay_area);

    let (search_title, search_style, border_color) = match topic_filter.mode {
        TopicInputMode::Editing => (
            format!(
                " Search Topics ({}) — Press Esc to finish search ",
                topic_filter.filtered_topics.len()
            ),
            Style::default().fg(Color::Yellow),
            Color::Yellow,
        ),
        TopicInputMode::Normal => (
            format!(
                " Search Topics ({}) — Press / to search ",
                topic_filter.filtered_topics.len()
            ),
            Style::default().fg(Color::DarkGray),
            Color::Cyan,
        ),
    };

    let selected_count = topic_filter.selected_topics.len();
    let title = if selected_count == 0 {
        " Topic Filter — j/k: navigate  /: search  Space/Enter: toggle  c: clear  Esc: close "
            .to_string()
    } else {
        format!(
            " Topic Filter ({} selected) — j/k: navigate  /: search  Space/Enter: toggle  c: clear  Esc: close ",
            selected_count
        )
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(title.as_str());

    let inner = block.inner(overlay_area);
    frame.render_widget(block, overlay_area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let search_widget = Paragraph::new(topic_filter.search_input.value())
        .style(search_style)
        .block(Block::default().borders(Borders::ALL).title(search_title));
    frame.render_widget(search_widget, layout[0]);

    if let TopicInputMode::Editing = topic_filter.mode {
        frame.set_cursor_position((
            layout[0].x + topic_filter.search_input.visual_cursor() as u16 + 1,
            layout[0].y + 1,
        ));
    }

    if topic_filter.all_topics.is_empty() {
        frame.render_widget(
            Paragraph::new("No topics available — ensure the problem list is fully loaded.")
                .style(Style::default().fg(Color::DarkGray)),
            layout[1],
        );
    } else if topic_filter.filtered_topics.is_empty() {
        frame.render_widget(
            Paragraph::new(format!(
                "No topics match \"{}\"",
                topic_filter.search_input.value()
            ))
            .style(Style::default().fg(Color::DarkGray)),
            layout[1],
        );
    } else {
        let items: Vec<ListItem> = topic_filter
            .filtered_topics
            .iter()
            .map(|t| {
                let (prefix, color) = if topic_filter.selected_topics.contains(t) {
                    ("[x] ", Color::Green)
                } else {
                    ("[ ] ", Color::White)
                };
                ListItem::new(format!("{}{}", prefix, t)).style(Style::default().fg(color))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, layout[1], &mut topic_filter.list_state);
    }

    let hint = if filtered_count == 0 {
        Paragraph::new("No problems match current filters").style(Style::default().fg(Color::Red))
    } else {
        Paragraph::new(format!("{} problems match", filtered_count))
            .style(Style::default().fg(Color::DarkGray))
    };
    frame.render_widget(hint, layout[2]);
}
