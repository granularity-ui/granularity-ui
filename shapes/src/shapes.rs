use std::rc::Rc;

use cosmic_text as text;
use derive_more::Constructor;

use crate::geometry::{Bounds, Matrix4};

#[derive(Debug)]
pub enum Shape {
    GlyphRun(GlyphRun),
}

#[derive(Debug)]
pub struct GlyphRun {
    pub model_matrix: Rc<Matrix4>,
    pub metrics: GlyphRunMetrics,
    pub glyphs: Vec<PositionedGlyph>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphRunMetrics {
    pub max_ascent: u32,
    pub max_descent: u32,
    pub width: u32,
}

impl GlyphRunMetrics {
    /// Size of the glyph run in font-size pixels.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.max_ascent + self.max_descent)
    }
}

#[derive(Debug, Clone, Constructor)]
pub struct PositionedGlyph {
    // This is for rendering the image of the glyph.
    pub key: text::CacheKey,
    pub hitbox_pos: (i32, i32),
    pub hitbox_width: f32,
}

impl PositionedGlyph {
    // The bounds enclosing a pixel at the offset of the hitbox
    pub fn pixel_bounds_at(&self, offset: (u32, u32)) -> Bounds {
        let x = self.hitbox_pos.0 + offset.0 as i32;
        let y = self.hitbox_pos.1 + offset.1 as i32;

        Bounds::new((x as f64, y as f64), ((x + 1) as f64, (y + 1) as f64))
    }
}
