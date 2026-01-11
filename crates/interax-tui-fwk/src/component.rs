//! Component traits for the TUI framework.
//!
//! This module defines the core traits for UI components.

use ratatui::{layout::Rect, Frame};

use crate::context::AppContext;
use crate::event::Event;

/// A UI component that can draw itself and handle events.
///
/// Components are the building blocks of your TUI application.
/// They can be composed together to create complex interfaces.
///
/// # Example
///
/// ```ignore
/// use interax_tui_fwk::{Component, Event, AppContext, KeyCode};
/// use ratatui::{Frame, layout::Rect, widgets::Paragraph};
///
/// struct Counter {
///     count: u32,
/// }
///
/// impl Component for Counter {
///     fn draw(&self, frame: &mut Frame, area: Rect) {
///         let text = format!("Count: {}", self.count);
///         frame.render_widget(Paragraph::new(text), area);
///     }
///
///     fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
///         if event.is_key(KeyCode::Up) {
///             self.count = self.count.saturating_add(1);
///             true
///         } else if event.is_key(KeyCode::Down) {
///             self.count = self.count.saturating_sub(1);
///             true
///         } else if event.is_quit() {
///             ctx.quit();
///             true
///         } else {
///             false
///         }
///     }
/// }
/// ```
pub trait Component: Send {
    /// Draw the component to the given frame area.
    ///
    /// This method should render the component's current state to the terminal.
    /// The `area` parameter defines the rectangular region where the component
    /// should draw itself.
    fn draw(&self, frame: &mut Frame, area: Rect);

    /// Handle an input event.
    ///
    /// The `ctx` parameter provides access to application-level controls
    /// like quitting the app or toggling mouse capture.
    ///
    /// Returns `true` if the event was consumed and should not propagate
    /// to other components, `false` otherwise.
    ///
    /// The default implementation does nothing and returns `false`.
    #[allow(unused_variables)]
    fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
        false
    }

    /// Called on each tick cycle if the app has a tick rate configured.
    ///
    /// The `ctx` parameter provides access to application-level controls.
    ///
    /// Use this for periodic updates like animations or polling.
    /// The default implementation does nothing.
    #[allow(unused_variables)]
    fn tick(&mut self, ctx: &mut AppContext) {}
}

/// The main UI trait for the root component of your application.
///
/// This trait extends `Component` with additional methods for handling
/// background task messages.
///
/// # Example
///
/// ```ignore
/// use interax_tui_fwk::{Component, MainUi, Event, AppContext};
/// use ratatui::{Frame, layout::Rect};
///
/// struct MyApp {
///     tabs: TabsComponent,
/// }
///
/// impl Component for MyApp {
///     fn draw(&self, frame: &mut Frame, area: Rect) {
///         self.tabs.draw(frame, area);
///     }
///
///     fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
///         if event.is_quit() {
///             ctx.quit();
///             return true;
///         }
///         self.tabs.handle_event(event, ctx)
///     }
/// }
///
/// impl MainUi for MyApp {}
/// ```
pub trait MainUi: Component {
    /// Handle a message from a background task.
    ///
    /// Override this method to process messages from your background tasks.
    /// The `task_name` identifies which task sent the message.
    /// The `ctx` parameter provides access to application-level controls.
    ///
    /// Returns `true` if a redraw is needed after processing the message.
    #[allow(unused_variables)]
    fn handle_task_message(
        &mut self,
        task_name: &str,
        message: Box<dyn std::any::Any + Send>,
        ctx: &mut AppContext,
    ) -> bool {
        false
    }
}

/// A boxed component for type-erased storage.
pub type BoxedComponent = Box<dyn Component>;

/// Extension trait for creating boxed components.
pub trait ComponentExt: Component + Sized + 'static {
    /// Box this component for type-erased storage.
    fn boxed(self) -> BoxedComponent {
        Box::new(self)
    }
}

impl<T: Component + 'static> ComponentExt for T {}
