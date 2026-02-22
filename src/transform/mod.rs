//! Coordinate transforms — mapping, clipping, and downsampling.

/// Cohen-Sutherland viewport clipping.
pub mod clip;
/// LTTB downsampling for large datasets.
pub mod downsample;
/// Data-to-pixel coordinate mapping.
pub mod mapper;

pub use clip::clip_line;
pub use downsample::{lttb_downsample, maybe_downsample};
pub use mapper::{CoordinateMapper, aligned_x_pixel_range, aligned_y_pixel_range};
