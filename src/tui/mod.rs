//! ratatui TUI runtime for leetrs.
//!
//! Provides the interactive problem browser. The top-level entry point is
//! [`run_tui`], which owns the terminal setup/teardown and re-opens the TUI
//! after Neovim closes so the user can pick another problem without restarting.
pub mod renderers;
pub mod screen;
mod utils;
pub mod widgets;

use crate::config::CONFIG;
use crate::tui::screen::selection_screen::{InputMode, SelectionScreen};
use crate::{picker::Picker, tui::screen::help_screen::HelpScreen};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
};
use screen::Screen;
use std::process::Command;
use std::{io, rc::Rc};

use crate::models::{Identifier, Language, ProblemSummary, UserDetail};

/// Which tab is currently displayed.
#[derive(Default)]
pub enum Tab {
    #[default]
    Selection,
    Help,
}

/// Holds the state of the application
/// Top-level application state shared across a single TUI session.
pub struct App {
    pub should_quit: bool,
    /// The full, shared problem list (reference-counted to avoid copying).
    pub problems: Rc<[ProblemSummary]>,
    pub tab: Tab,
    pub selection_screen: SelectionScreen,
    pub help_screen: HelpScreen,
    /// Slug of the problem the user pressed Enter on, if any.
    pub selected_problem: Option<String>,
    pub user_detail: Option<UserDetail>,
    /// One-shot message shown in a modal popup until dismissed.
    pub popup_message: Option<String>,
}

/// Actions that a [`Screen`] can return to the main event loop.
pub enum Action {
    Quit,
    /// The user selected a problem; carries its slug.
    Select(String),
    /// Display a one-shot modal popup with the given message.
    ShowMessage(String),
    /// Dismiss the currently active popup.
    DismissPopup,
    /// Open the given URL in the system browser.
    Open(String),
}

impl App {
    pub fn new(problems: Rc<[ProblemSummary]>, user_detail: Option<UserDetail>) -> Self {
        let mut list_state = ListState::default();
        if !problems.is_empty() {
            list_state.select(Some(0)); // Start by highlighting the first item
        }

        Self {
            should_quit: false,
            selection_screen: SelectionScreen::new(Rc::clone(&problems), user_detail.clone()),
            problems,
            tab: Tab::default(),
            selected_problem: None,
            help_screen: HelpScreen::new(),
            user_detail,
            popup_message: None,
        }
    }

    pub fn switch(&mut self) {
        self.tab = match self.tab {
            Tab::Help => Tab::Selection,
            Tab::Selection => Tab::Help,
        }
    }
}

/// RAII guard for the terminal alternate screen / raw mode.
///
/// Entering the terminal and restoring it are always done as a pair. This
/// guard ensures the cleanup code runs even if the TUI panics mid-session,
/// preventing the user's shell from being left in raw mode.
struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalGuard {
    fn enter() -> anyhow::Result<Self> {
        enable_raw_mode().map_err(anyhow::Error::from)?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).map_err(anyhow::Error::from)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best-effort restoration — ignore errors during panic unwinding.
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

/// The main entry point for the TUI
/// Initialises [`App`], then enters the TUI event loop.
///
/// After the user selects a problem the terminal is restored, Neovim is
/// launched, and the TUI is re-entered so the user can pick another problem
/// without restarting the process.
pub async fn run_tui(
    problems: Rc<[ProblemSummary]>,
    picker: Picker,
    user_detail: Option<UserDetail>,
    language: &Option<Language>,
) -> anyhow::Result<()> {
    let mut app = App::new(problems, user_detail);
    let _result = loop {
        let mut guard = TerminalGuard::enter()?;
        let result = run_app(guard.terminal_mut(), &mut app).await;
        // Guard drops here, restoring the terminal
        drop(guard);

        match result {
            Ok(Some(problem)) => {
                pick_and_open_editor(&picker, &Identifier::String(problem), language).await;
                app.selection_screen.input_mode = InputMode::Normal;
                app.should_quit = false;
                app.selected_problem = None;
            }
            Ok(None) => break Ok(()),
            Err(e) => break Err(anyhow::Error::from(e)),
        }
    };
    Ok(())
}

/// The Event Loop
/// Drives rendering and keyboard events for a single TUI session.
///
/// Returns `Ok(Some(slug))` when the user selects a problem, `Ok(None)` when
/// they quit, and `Err` on I/O failure.
async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<Option<String>> {
    loop {
        let screen: &mut dyn Screen = match app.tab {
            Tab::Selection => &mut app.selection_screen,
            Tab::Help => &mut app.help_screen,
        };

        let _ = terminal.draw(|f| {
            screen.render(f);
            if let Some(popup_message) = &app.popup_message {
                let centered_area = f
                    .area()
                    .centered(Constraint::Percentage(60), Constraint::Percentage(20));
                f.render_widget(Clear, centered_area);
                let layout = Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(centered_area);
                let popup_block = Block::bordered().title("Alert");
                let paragraph = Paragraph::new(popup_message.as_str()).block(popup_block);
                f.render_widget(paragraph, layout[0]);
                let hint = Paragraph::new("Press Enter or Esc to close");
                f.render_widget(hint, layout[1]);
            }
        });

        // Poll for keystrokes (non-blocking)
        if event::poll(std::time::Duration::from_millis(50))? {
            let event = event::read()?;
            if let Event::Key(key) = event
                && key.kind == KeyEventKind::Press
            {
                if app.popup_message.is_some() {
                    match key.code {
                        KeyCode::Enter | KeyCode::Esc => {
                            app.popup_message = None;
                        }
                        _ => {}
                    }
                    continue;
                }
                match key.code {
                    KeyCode::Tab => app.switch(),
                    KeyCode::Char('?') => app.tab = Tab::Help,
                    _ => {
                        if let Some(action) = screen.event_loop(&key) {
                            match action {
                                Action::Quit => {
                                    app.should_quit = true;
                                }
                                Action::Select(problem) => {
                                    app.selected_problem = Some(problem);
                                    app.should_quit = true;
                                }
                                Action::ShowMessage(msg) => {
                                    app.popup_message = Some(msg);
                                    app.should_quit = false;
                                }
                                Action::DismissPopup => {
                                    app.popup_message = None;
                                }
                                Action::Open(url) => {
                                    let _ = open::that(url);
                                }
                            }
                        }
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(app.selected_problem.clone());
        }
    }
}

pub async fn pick_and_open_editor(
    picker: &Picker,
    identifier: &Identifier,
    language: &Option<Language>,
) {
    match picker.pick(identifier, language).await {
        Ok((code, desc)) => {
            let config = CONFIG.get().expect("Failed to initialise config");
            let editor = config.editor.as_deref().unwrap_or("nvim");
            let show_description = config.show_description.unwrap_or(true);

            println!("🚀 launching {}...", editor);

            let status = if show_description {
                if editor.contains("nvim") || editor.contains("vim") {
                    Command::new(editor)
                        .arg(&desc)
                        .arg("-c")
                        .arg(format!("vsplit {}", code))
                        .status()
                } else {
                    Command::new(editor).arg(&desc).arg(&code).status()
                }
            } else {
                Command::new(editor).arg(&code).status()
            };

            match status {
                Ok(exit_status) if exit_status.success() => {
                    println!("\n👋 {} closed.", editor);
                }
                Ok(exit_status) => {
                    eprintln!("⚠️ {} exited with an error code: {}", editor, exit_status);
                }
                Err(e) => {
                    eprintln!(
                        "❌ failed to launch {}. is it installed and in your path? error: {}",
                        editor, e
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to pick problem: {}", e);
            println!("\nPress Enter to return to TUI...");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
        }
    }
}
