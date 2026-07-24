//! Search bar widget — wraps `tui_input::Input` for the search field.
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::{Input, backend::crossterm::EventHandler};

/// Manages the search input field state.
pub struct SearchBar {
    pub input: Input,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn visual_cursor(&self) -> usize {
        self.input.visual_cursor()
    }

    /// Handles a key event in editing mode. Returns `true` if the input changed.
    pub fn handle_key(&mut self, key_event: &KeyEvent) -> bool {
        // Skip control keys that are not text input
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return false;
        }
        match key_event.code {
            KeyCode::Esc | KeyCode::Enter => false,
            _ => {
                self.input.handle_event(&Event::Key(*key_event));
                true
            }
        }
    }

    pub fn clear(&mut self) {
        self.input = Input::default();
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}
