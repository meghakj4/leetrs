//! Popup overlay — a modal alert Screen that dismisses on Enter/Esc.
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Constraint,
    layout::Direction,
    layout::Layout,
    widgets::{Block, Clear, Paragraph},
};

use crate::tui::{Action, screen::Screen};

/// A full-screen modal popup that renders a message and dismisses on Enter/Esc.
///
/// Used to show premium-gate messages and other one-shot alerts without
/// embedding popup logic in the main event loop.
pub struct PopupOverlay {
    pub message: String,
}

impl Screen for PopupOverlay {
    fn render(&mut self, frame: &mut Frame) {
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(centered_area);
        let popup_block = Block::bordered().title("Alert");
        let paragraph = Paragraph::new(self.message.as_str()).block(popup_block);
        frame.render_widget(paragraph, layout[0]);
        let hint = Paragraph::new("Press Enter or Esc to close");
        frame.render_widget(hint, layout[1]);
    }

    fn event_loop(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Enter | KeyCode::Esc => Some(Action::DismissPopup),
            _ => None,
        }
    }
}
