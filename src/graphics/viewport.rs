
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    /// Coordinates in pixels of the top-left hand corner of the viewport.
    pub origin: [f32; 2],

    /// Dimensions in pixels of the viewport.
    pub dimensions: [f32; 2],

    /// Minimum and maximum values of the depth.
    ///
    /// The values `0.0` to `1.0` of each vertex's Z coordinate will be mapped to this
    /// `depth_range` before being compared to the existing depth value.
    ///
    /// This is equivalents to `glDepthRange` in OpenGL, except that OpenGL uses the Z coordinate
    /// range from `-1.0` to `1.0` instead.
    pub depth_range: Range<f32>,
}
