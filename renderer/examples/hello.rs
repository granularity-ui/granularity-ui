use std::rc::Rc;

use cosmic_text as text;
use derive_more::Constructor;
use granularity_geometry::{Camera, Matrix4, Vector3};
use granularity_shapes::{GlyphRun, GlyphRunMetrics, PositionedGlyph, Shape};
use winit::event::{VirtualKeyCode, WindowEvent};

use granularity_shell::{self as shell, Shell};

struct Application {
    camera: Camera,
    hello_world: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let fovy: f64 = 45.0;
    let camera_distance = 1.0 / (fovy / 2.0).to_radians().tan();

    let camera = Camera::new((0.0, 0.0, camera_distance), (0.0, 0.0, 0.0));

    let hello_world = "Hello, world!";

    let application = Application {
        camera,
        hello_world: hello_world.to_string(),
    };

    let _ = shell::run(application).await;
}

impl shell::Application for Application {
    fn update(&mut self, window_event: WindowEvent<'static>) {
        if let WindowEvent::KeyboardInput { input, .. } = window_event {
            if input.state == winit::event::ElementState::Pressed {
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Left) => self.camera.eye += Vector3::new(0.1, 0.0, 0.0),
                    Some(VirtualKeyCode::Right) => self.camera.eye -= Vector3::new(0.1, 0.0, 0.0),
                    Some(VirtualKeyCode::Up) => self.camera.eye += Vector3::new(0.0, 0.0, 0.1),
                    Some(VirtualKeyCode::Down) => self.camera.eye -= Vector3::new(0.0, 0.0, 0.1),
                    _ => {}
                }
            } else {
                {}
            }
        }
    }

    fn render(&self, shell: &mut Shell) -> (Camera, Vec<Shape>) {
        const FONT_SIZE: f32 = 100.0;

        let text = shape_text(&mut shell.font_system, &self.hello_world, FONT_SIZE);

        let shapes = vec![Shape::GlyphRun(
            text.into_glyph_run(Rc::new(shell.pixel_matrix())),
        )];

        (self.camera, shapes)
    }
}

#[derive(Debug, Constructor)]
pub struct ShapedText {
    pub metrics: GlyphRunMetrics,
    pub glyphs: Vec<PositionedGlyph>,
}

impl ShapedText {
    pub fn into_glyph_run(self, matrix: Rc<Matrix4>) -> GlyphRun {
        GlyphRun {
            glyphs: self.glyphs,
            model_matrix: matrix.clone(),
            metrics: self.metrics,
        }
    }
}

fn shape_text(font_system: &mut text::FontSystem, text: &str, font_size: f32) -> ShapedText {
    let mut buffer = text::BufferLine::new(
        text,
        text::AttrsList::new(text::Attrs::new()),
        text::Shaping::Advanced,
    );
    let line = &buffer.layout(font_system, font_size, f32::MAX, text::Wrap::None)[0];
    let line_glyphs = &line.glyphs;
    let placed = place_glyphs(line_glyphs);
    let metrics = GlyphRunMetrics {
        max_ascent: line.max_ascent as u32,
        max_descent: line.max_descent as u32,
        width: line.w.ceil() as u32,
    };

    ShapedText::new(metrics, placed)
}

const RENDER_SUBPIXEL: bool = false;

fn place_glyphs(glyphs: &[text::LayoutGlyph]) -> Vec<PositionedGlyph> {
    glyphs
        .iter()
        .map(|glyph| {
            let fractional_pos = if RENDER_SUBPIXEL {
                (glyph.x, glyph.y)
            } else {
                (glyph.x.round(), glyph.y.round())
            };

            let (cc, x, y) = text::CacheKey::new(
                glyph.font_id,
                glyph.glyph_id,
                glyph.font_size,
                fractional_pos,
            );
            // Note: hitbox with is fractional, but does not change with / without subpixel
            // rendering.
            PositionedGlyph::new(cc, (x, y), glyph.w)
        })
        .collect()
}
