//! # interax-tui-fwk
//!
//! An async, event-driven TUI framework built on ratatui and tokio.
//!
//! This framework provides a clean architecture for building terminal user interfaces
//! with minimal CPU usage. It only redraws in response to events, making it ideal
//! for applications that need to be "quiet" and power-efficient.
//!
//! ## Features
//!
//! - **Event-driven**: No polling, only responds to terminal events and task messages
//! - **Async tasks**: Background tasks communicate via typed message channels
//! - **Builder pattern**: Clean, composable application setup
//! - **Minimal allocations**: Designed for efficiency in hot paths
//! - **Runtime control**: Toggle mouse capture, quit, etc. via `AppContext`
//!
//! ## Quick Start
//!
//! ```ignore
//! use interax_tui_fwk::{AppBuilder, Component, MainUi, Event, AppContext};
//! use ratatui::{Frame, layout::Rect, widgets::Paragraph};
//!
//! struct MyApp {
//!     counter: u32,
//! }
//!
//! impl Component for MyApp {
//!     fn draw(&self, frame: &mut Frame, area: Rect) {
//!         let text = format!("Counter: {}", self.counter);
//!         frame.render_widget(Paragraph::new(text), area);
//!     }
//!
//!     fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
//!         if event.is_quit() {
//!             ctx.quit();
//!             return true;
//!         }
//!         false
//!     }
//! }
//!
//! impl MainUi for MyApp {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app = AppBuilder::new()
//!         .main_ui(MyApp { counter: 0 })
//!         .mouse_capture(true)  // Enable mouse events (default)
//!         .build()?;
//!
//!     app.run().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Background Tasks
//!
//! Tasks run asynchronously and send typed messages to the UI:
//!
//! ```ignore
//! use interax_tui_fwk::{Task, TaskContext, TaskSender};
//!
//! struct TickerTask;
//!
//! #[derive(Debug)]
//! struct Tick(u64);
//!
//! impl Task for TickerTask {
//!     type Message = Tick;
//!
//!     async fn run(self, sender: TaskSender<Self::Message>, mut ctx: TaskContext) {
//!         let mut count = 0;
//!         let mut interval = tokio::time::interval(Duration::from_secs(1));
//!
//!         loop {
//!             tokio::select! {
//!                 _ = interval.tick() => {
//!                     count += 1;
//!                     if sender.send(Tick(count)).await.is_err() {
//!                         break;
//!                     }
//!                 }
//!                 _ = ctx.cancelled() => break,
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Runtime Control
//!
//! Use `AppContext` to control the application at runtime:
//!
//! ```ignore
//! fn handle_event(&mut self, event: &Event, ctx: &mut AppContext) -> bool {
//!     // Toggle mouse capture with 'm' key
//!     if event.is_key(KeyCode::Char('m')) {
//!         let enabled = ctx.mouse_capture_enabled();
//!         ctx.set_mouse_capture(!enabled).ok();
//!         return true;
//!     }
//!     
//!     // Quit with 'q'
//!     if event.is_key(KeyCode::Char('q')) {
//!         ctx.quit();
//!         return true;
//!     }
//!     
//!     false
//! }
//! ```

pub mod app;
pub mod bus;
pub mod component;
pub mod context;
pub mod event;
pub mod task;
pub mod terminal;

// Re-export main types at crate root for convenience
pub use app::{App, AppBuilder, AppError, BuildError};
pub use bus::{MessageBus, SendError, TaskMessage, TaskSender, TrySendError};
pub use component::{BoxedComponent, Component, ComponentExt, MainUi};
pub use context::AppContext;
pub use event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
pub use task::{Task, TaskContext, TaskHandle};
pub use terminal::{install_panic_hook, Terminal, TerminalConfig, TerminalError};

// Conditionally re-export blocking task helpers
#[cfg(feature = "blocking-tasks")]
pub use task::{spawn_blocking, spawn_blocking_unwrap};

// Re-export ratatui types that users commonly need
pub use ratatui::{layout::Rect, Frame};
