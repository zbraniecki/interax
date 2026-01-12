//! Application contexts for event handlers and drawing.
//!
//! This module provides context types that are passed to event handlers
//! and draw methods, allowing components to control application behavior
//! and access shared state.

use ratatui::{layout::Rect, Frame};

use crate::tabs::{TabInfo, TabManager};
use crate::terminal::{Terminal, TerminalError};

/// Context passed to event handlers for controlling the application.
///
/// `AppContext` provides methods to:
/// - Request application quit
/// - Toggle mouse capture
/// - Access terminal state
/// - Control tab selection
///
/// # Example
///
/// ```ignore
/// use interax_tui_fwk::{Component, Event, AppContext, KeyCode};
///
/// impl Component for MyApp {
///     fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
///         if event.is_quit() {
///             ctx.quit();
///             return true;
///         }
///         
///         // Switch tabs with Tab key
///         if event.is_key(KeyCode::Tab) {
///             ctx.tabs().select_next();
///             return true;
///         }
///         
///         false
///     }
/// }
/// ```
pub struct AppContext<'a> {
    pub(crate) terminal: &'a mut Terminal,
    pub(crate) tab_manager: &'a mut TabManager,
    pub(crate) should_quit: bool,
}

impl<'a> AppContext<'a> {
    /// Create a new application context.
    pub(crate) fn new(terminal: &'a mut Terminal, tab_manager: &'a mut TabManager) -> Self {
        Self {
            terminal,
            tab_manager,
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
    pub fn set_mouse_capture(&mut self, enabled: bool) -> Result<(), TerminalError> {
        self.terminal.set_mouse_capture(enabled)
    }

    /// Get the terminal size.
    pub fn terminal_size(&self) -> Result<Rect, TerminalError> {
        self.terminal.size()
    }

    /// Access tab controls for event handling.
    ///
    /// Use this to select tabs, navigate between tabs, etc.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Select next tab
    /// ctx.tabs().select_next();
    ///
    /// // Select a specific tab by ID
    /// ctx.tabs().select_by_id("settings");
    ///
    /// // Select previous tab
    /// ctx.tabs().select_prev();
    /// ```
    #[inline]
    pub fn tabs(&mut self) -> TabsEventContext<'_> {
        TabsEventContext {
            manager: self.tab_manager,
        }
    }
}

/// Tab controls available during event handling.
///
/// Access this through `AppContext::tabs()`.
pub struct TabsEventContext<'a> {
    manager: &'a mut TabManager,
}

impl TabsEventContext<'_> {
    /// Get the list of all registered tabs.
    pub fn list(&self) -> Vec<TabInfo> {
        self.manager.list()
    }

    /// Get the index of the currently active tab.
    pub fn active_index(&self) -> usize {
        self.manager.active_index()
    }

    /// Get the ID of the currently active tab, if any.
    pub fn active_id(&self) -> Option<&str> {
        self.manager.active_tab().map(|t| t.id())
    }

    /// Select a tab by index.
    ///
    /// Returns `true` if the tab was selected, `false` if the index is invalid
    /// or the tab is disabled.
    pub fn select(&mut self, index: usize) -> bool {
        self.manager.select(index)
    }

    /// Select a tab by its unique ID.
    ///
    /// Returns `true` if the tab was found and selected.
    pub fn select_by_id(&mut self, id: &str) -> bool {
        self.manager.select_by_id(id)
    }

    /// Select the next enabled tab.
    ///
    /// Wraps around to the first tab if at the end.
    pub fn select_next(&mut self) -> bool {
        self.manager.select_next()
    }

    /// Select the previous enabled tab.
    ///
    /// Wraps around to the last tab if at the beginning.
    pub fn select_prev(&mut self) -> bool {
        self.manager.select_prev()
    }

    /// Check if there are any registered tabs.
    pub fn is_empty(&self) -> bool {
        self.manager.is_empty()
    }

    /// Get the number of registered tabs.
    pub fn len(&self) -> usize {
        self.manager.len()
    }

    /// Check if a tab is enabled.
    ///
    /// A tab is enabled if both:
    /// - The tab's own `is_enabled()` returns true
    /// - The tab has not been disabled via `set_enabled(id, false)`
    pub fn is_enabled(&self, id: &str) -> bool {
        self.manager.is_enabled(id)
    }

    /// Enable or disable a tab by ID.
    ///
    /// When `enabled` is `false`, the tab is disabled and cannot be selected.
    /// When `enabled` is `true`, the tab reverts to its own `is_enabled()` state.
    ///
    /// Returns `true` if the tab was found.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Disable the settings tab
    /// ctx.tabs().set_enabled("settings", false);
    ///
    /// // Re-enable it
    /// ctx.tabs().set_enabled("settings", true);
    ///
    /// // Toggle
    /// let enabled = ctx.tabs().is_enabled("settings");
    /// ctx.tabs().set_enabled("settings", !enabled);
    /// ```
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        self.manager.set_enabled(id, enabled)
    }
}

/// Context passed to draw methods for rendering.
///
/// `DrawContext` provides access to:
/// - Tab bar and content drawing
/// - Tab information
///
/// # Example
///
/// ```ignore
/// use interax_tui_fwk::{Component, DrawContext};
/// use ratatui::{Frame, layout::Rect};
///
/// impl Component for MyApp {
///     fn draw(&self, frame: &mut Frame, area: Rect, ctx: &DrawContext) {
///         // Draw the tab bar at the top
///         let tab_bar_height = 2;
///         let (tab_area, content_area) = split_vertical(area, tab_bar_height);
///         
///         ctx.tabs().draw_tabbar(frame, tab_area);
///         ctx.tabs().draw_content(frame, content_area);
///     }
/// }
/// ```
pub struct DrawContext<'a> {
    pub(crate) tab_manager: &'a TabManager,
}

impl<'a> DrawContext<'a> {
    /// Create a new draw context.
    pub(crate) fn new(tab_manager: &'a TabManager) -> Self {
        Self { tab_manager }
    }

    /// Access tab information and drawing methods.
    #[inline]
    pub fn tabs(&self) -> TabsDrawContext<'_> {
        TabsDrawContext {
            manager: self.tab_manager,
        }
    }
}

/// Tab drawing context available during rendering.
///
/// Access this through `DrawContext::tabs()`.
pub struct TabsDrawContext<'a> {
    manager: &'a TabManager,
}

impl TabsDrawContext<'_> {
    /// Get the list of all registered tabs.
    pub fn list(&self) -> Vec<TabInfo> {
        self.manager.list()
    }

    /// Get the index of the currently active tab.
    pub fn active_index(&self) -> usize {
        self.manager.active_index()
    }

    /// Get the ID of the currently active tab, if any.
    pub fn active_id(&self) -> Option<&str> {
        self.manager.active_tab().map(|t| t.id())
    }

    /// Check if there are any registered tabs.
    pub fn is_empty(&self) -> bool {
        self.manager.is_empty()
    }

    /// Get the number of registered tabs.
    pub fn len(&self) -> usize {
        self.manager.len()
    }

    /// Draw the tab bar to the given area.
    ///
    /// This renders a horizontal tab bar showing all registered tabs,
    /// with the active tab highlighted.
    pub fn draw_tabbar(&self, frame: &mut Frame, area: Rect) {
        self.manager.draw_tabbar(frame, area);
    }

    /// Draw the content of the currently active tab.
    ///
    /// This calls the active tab's `draw` method with the given area.
    pub fn draw_content(&self, frame: &mut Frame, area: Rect) {
        self.manager.draw_content(frame, area);
    }
}
