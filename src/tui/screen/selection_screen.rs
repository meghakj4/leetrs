//! The main problem-selection screen shown when the TUI starts.
//!
//! `SelectionScreen` is now a thin orchestrator that delegates to focused
//! sub-widgets: [`FilterState`], [`SearchBar`], [`ProblemTable`], and
//! [`PremiumGate`]. Rendering utilities live in [`renderers`] and
//! [`topic_overlay`].
use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Table},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    models::{ProblemSummary, UserDetail},
    tui::{
        Action,
        renderers::render_problem_row,
        screen::Screen,
        widgets::{
            filter_state::FilterState, premium_gate::PremiumGate, problem_table::ProblemTable,
            search_bar::SearchBar, topic_overlay::render_topic_overlay,
        },
    },
};

/// Keyboard input mode for the selection screen.
pub enum InputMode {
    Editing,
    Normal,
    TopicFilter,
}

/// The problem-list screen: a searchable, filterable table of all problems.
pub struct SelectionScreen {
    /// The full, immutable problem list shared with [`App`].
    pub all_problems: Rc<[ProblemSummary]>,
    /// Indices into `all_problems` that survive the current search/filter.
    pub filtered_problems: Vec<usize>,
    pub table: ProblemTable,
    pub search: SearchBar,
    pub filters: FilterState,
    pub input_mode: InputMode,
    /// Tracks the previous key for `gg` (jump-to-top) detection.
    pub previous_key: Option<KeyCode>,
    pub user_detail: Option<UserDetail>,
}

impl Screen for SelectionScreen {
    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let title = format!(" Search ({} matches) ", self.filtered_problems.len());
        let input_widget = Paragraph::new(self.search.value())
            .style(match self.input_mode {
                InputMode::Editing => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title(title));
        frame.render_widget(input_widget, chunks[0]);

        if let InputMode::Editing = self.input_mode {
            frame.set_cursor_position((
                chunks[0].x + self.search.visual_cursor() as u16 + 1,
                chunks[0].y + 1,
            ));
        }

        let table_title = self.build_table_title();
        let header_cells = ["ID", "Name", "Acceptance", "Topics", "Premium?", "Done"]
            .into_iter()
            .map(|h| ratatui::widgets::Cell::from(h).style(Style::default().fg(Color::Yellow)));
        let header = ratatui::widgets::Row::new(header_cells).style(Style::default());

        let rows: Vec<_> = self
            .filtered_problems
            .iter()
            .map(|&p| render_problem_row(&self.all_problems[p]))
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(6),
                Constraint::Percentage(45),
                Constraint::Min(10),
                Constraint::Fill(10),
                Constraint::Min(8),
                Constraint::Length(6),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .title(table_title.as_str())
                .borders(Borders::ALL),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .highlight_symbol(">> ");

        frame.render_stateful_widget(table, chunks[1], &mut self.table.state);

        // Bottom status bar
        let bottom_bar = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(chunks[2]);

        let (instruction_text, instruction_style) = match self.input_mode {
            InputMode::Normal => (
                "Press '/' to search, 'j'/'k' to scroll, 'Enter' to select, 'o' to open in \
                browser, 'q' to quit.",
                Style::default().fg(Color::DarkGray),
            ),
            InputMode::Editing => (
                "Type to filter, press 'Esc' to return to list, press 'Enter' to select.",
                Style::default().fg(Color::Yellow),
            ),
            InputMode::TopicFilter => (
                "j/k: navigate   Space: toggle   c: clear all   Esc/Enter: close",
                Style::default().fg(Color::Cyan),
            ),
        };
        frame.render_widget(
            Paragraph::new(instruction_text).style(instruction_style),
            bottom_bar[0],
        );

        if let InputMode::Normal = self.input_mode {
            frame.render_widget(
                Paragraph::new("1: Easy  2: Medium  3: Hard  4: All  |  t: Topic filter")
                    .style(Style::default().fg(Color::DarkGray)),
                bottom_bar[1],
            );
        }

        let topic_status_widget = if self.filters.topics.selected_topics.is_empty() {
            Paragraph::new("Press ? to view help.").style(Style::default().fg(Color::DarkGray))
        } else {
            let mut names: Vec<&str> = self
                .filters
                .topics
                .selected_topics
                .iter()
                .map(|s| s.as_str())
                .collect();
            names.sort();
            let display = if names.len() <= 3 {
                format!("Topics: {}", names.join(", "))
            } else {
                format!(
                    "Topics: {}, ... (+{} more)",
                    names[..2].join(", "),
                    names.len() - 2
                )
            };
            Paragraph::new(display).style(Style::default().fg(Color::Cyan))
        };
        frame.render_widget(topic_status_widget, bottom_bar[2]);

        if let InputMode::TopicFilter = self.input_mode {
            render_topic_overlay(
                frame,
                &mut self.filters.topics,
                self.filtered_problems.len(),
            );
        }
    }

    fn event_loop(&mut self, key_event: &KeyEvent) -> Option<Action> {
        if let InputMode::TopicFilter = self.input_mode {
            return self.handle_topic_filter_key(key_event);
        }

        if let KeyCode::Enter = key_event.code
            && let Some(i) = self.table.state.selected()
            && !self.filtered_problems.is_empty()
        {
            let index = self.filtered_problems[i];
            let problem = &self.all_problems[index];
            match PremiumGate::can_access(problem, self.user_detail.as_ref()) {
                Err(msg) => return Some(Action::ShowMessage(msg)),
                Ok(()) => return Some(Action::Select(problem.slug.clone())),
            }
        }

        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Some(Action::Quit),
                KeyCode::Down | KeyCode::Char('j') => self.table.next(),
                KeyCode::Up | KeyCode::Char('k') => self.table.previous(),
                KeyCode::Left | KeyCode::Char('h') => self.table.state.select_next_column(),
                KeyCode::Right | KeyCode::Char('l') => self.table.state.select_previous_column(),
                KeyCode::Char('/') => self.input_mode = InputMode::Editing,
                KeyCode::Char('t') => self.input_mode = InputMode::TopicFilter,
                KeyCode::Char('o') => {
                    if let Some(i) = self.table.state.selected()
                        && !self.filtered_problems.is_empty()
                    {
                        let index = self.filtered_problems[i];
                        let selected = &self.all_problems[index];
                        let url = format!("https://leetcode.com/problems/{}", selected.slug);
                        self.input_mode = InputMode::Normal;
                        return Some(Action::Open(url));
                    }
                }
                KeyCode::Char('g') => {
                    if let Some(prev_key) = self.previous_key
                        && prev_key == KeyCode::Char('g')
                    {
                        self.table.select_first();
                    }
                }
                KeyCode::Char('G') => {
                    self.table.select_last();
                }
                KeyCode::Char('d') => {
                    self.table.scroll_down(10);
                }
                KeyCode::Char('u') => {
                    self.table.scroll_up(10);
                }
                KeyCode::Char(c) => {
                    if let Some(number) = c.to_digit(10) {
                        self.switch_difficulty(number as u8);
                    }
                }
                _ => {}
            },

            InputMode::Editing => match key_event.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char('j') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.table.next();
                }
                KeyCode::Char('k') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.table.previous();
                }
                _ => {
                    self.search.input.handle_event(&Event::Key(*key_event));
                    self.apply_filters();
                }
            },

            InputMode::TopicFilter => unreachable!(),
        }
        self.previous_key = Some(key_event.code);
        None
    }
}

impl SelectionScreen {
    pub fn new(problems: Rc<[ProblemSummary]>, user_detail: Option<UserDetail>) -> Self {
        let len = problems.len();
        let filters = FilterState::new();
        Self {
            filtered_problems: (0..len).collect(),
            table: ProblemTable::new(len),
            search: SearchBar::new(),
            filters,
            all_problems: problems,
            input_mode: InputMode::Normal,
            previous_key: None,
            user_detail,
        }
    }

    /// Sets the difficulty filter and re-applies all active filters.
    pub fn switch_difficulty(&mut self, difficulty: u8) {
        self.filters.set_difficulty(difficulty);
        self.apply_filters();
    }

    /// Rebuilds `filtered_problems` by applying all active filters in one pass.
    pub fn apply_filters(&mut self) {
        let query = self.search.value().to_string();
        self.filtered_problems = self.filters.apply(&self.all_problems, &query);
        self.table.update_len(self.filtered_problems.len());
    }

    fn handle_topic_filter_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.filters.topics.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.filters.topics.previous();
            }
            KeyCode::Char(' ') => {
                self.filters.topics.toggle_current();
                self.apply_filters();
            }
            KeyCode::Char('c') => {
                self.filters.topics.clear();
                self.apply_filters();
            }
            _ => {}
        }
        None
    }

    fn build_table_title(&self) -> String {
        let diff_part = match self.filters.difficulty {
            Some(1) => " (Easy)",
            Some(2) => " (Medium)",
            Some(3) => " (Hard)",
            _ => "",
        };

        let topic_part = match self.filters.topics.selected_topics.len() {
            0 => String::new(),
            n => {
                let mut names: Vec<&str> = self
                    .filters
                    .topics
                    .selected_topics
                    .iter()
                    .map(|s| s.as_str())
                    .collect();
                names.sort();
                if n <= 2 {
                    format!(" [{}]", names.join(", "))
                } else {
                    format!(" [{}, +{}]", names[..2].join(", "), n - 2)
                }
            }
        };

        format!(" Problems{}{} ", diff_part, topic_part)
    }
}
