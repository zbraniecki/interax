//! Application context for event handlers.
//!
//! This module provides the `AppContext` type that is passed to event
//! handlers, allowing them to control application behavior at runtime.

use crate::terminal::{Terminal, TerminalError};

/// Context passed to event handlers for controlling the application.
///
/// `AppContext` provides methods to:
/// - Request application quit
/// - Toggle mouse capture
/// - Access terminal state
///
/// # Example
///
/// ```ignore
/// use interax_tui_fwk::{Component, Event, AppContext};
///
/// impl Component for MyApp {
///     fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
///         if event.is_quit() {
///             ctx.quit();
///             return true;
///         }
///         
///         // Toggle mouse on 'm' key
///         if event.is_key(KeyCode::Char('m')) {
///             let current = ctx.mouse_capture_enabled();
///             ctx.set_mouse_capture(!current).ok();
///             return true;
///         }
///         
///         false
///     }
/// }
/// ```
pub struct AppContext<'a> {
    pub(crate) terminal: &'a mut Terminal,
    pub(crate) should_quit: bool,
}

impl<'a> AppContext<'a> {
    /// Create a new application context.
    pub(crate) fn new(terminal: &'a mut Terminal) -> Self {
        Self {
            terminal,
            should_quit: false,
        }
    }

    /// Request the application to quit.
    ///
    /// The application will exit gracefully after the current event
    /// is processed.
    #[inline]
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Check if quit has been requested.
    #[inline]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Check if mouse capture is currently enabled.
    #[inline]
    pub fn mouse_capture_enabled(&self) -> bool {
        self.terminal.mouse_capture_enabled()
    }

    /// Enable or disable mouse capture at runtime.
    ///
    /// Returns an error if the terminal operation fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Disable mouse capture
    /// ctx.set_mouse_capture(false)?;
    ///
    /// // Toggle mouse capture
    /// let current = ctx.mouse_capture_enabled();
    /// ctx.set_mouse_capture(!current)?;
    /// ```
    pub fn set_mouse_capture(&mut self, enabled: bool) -> Result<(), TerminalError> {
        self.terminal.set_mouse_capture(enabled)
    }

    /// Get the terminal size.
    pub fn terminal_size(&self) -> Result<ratatui::layout::Rect, TerminalError> {
        self.terminal.size()
    }
}
