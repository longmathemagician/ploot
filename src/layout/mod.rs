//! Layout engine — tick generation, text grid, and frame rendering.

/// Space allocation and frame rendering.
pub mod engine;
/// Heckbert's "nice numbers" tick generation.
pub mod nice_numbers;
/// Character grid for text compositing.
pub mod text;

pub use engine::{Layout, LayoutConfig, compute_layout, render_frame};
pub use nice_numbers::{TickSet, generate_ticks};
pub use text::TextGrid;
