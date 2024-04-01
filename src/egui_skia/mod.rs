pub(crate) mod painter;

use egui::Context;
use skia_safe::Canvas;

use crate::egui_skia::painter::Painter;

pub struct RasterizeOptions {
    pub pixels_per_point: f32,
    /// The number of frames to render before a screenshot is taken.
    /// Default is 2, so egui will be able to display windows
    pub frames_before_screenshot: usize,
}

impl Default for RasterizeOptions {
    fn default() -> Self {
        Self {
            pixels_per_point: 1.0,
            frames_before_screenshot: 2,
        }
    }
}
/// Convenience wrapper for using [`egui`] from a [`skia`] app.
pub struct EguiSkia {
    pub egui_ctx: Context,
    pub painter: Painter,

    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,
    pixels_per_point: f32,
}

impl EguiSkia {
    pub fn new(pixels_per_point: f32) -> Self {
        let painter = Painter::new();
        Self {
            egui_ctx: Default::default(),
            painter,
            shapes: Default::default(),
            textures_delta: Default::default(),
            pixels_per_point,
        }
    }

    /// Returns a duration after witch egui should repaint.
    ///
    /// Call [`Self::paint`] later to paint.
    pub fn run(
        &mut self,
        input: egui::RawInput,
        run_ui: impl FnMut(&Context),
    ) -> egui::PlatformOutput {
        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point: _,
            // TODO: How to handle multiple outputs
            viewport_output: _,
        } = self.egui_ctx.run(input, run_ui);

        self.shapes = shapes;
        self.textures_delta.append(textures_delta);

        platform_output
    }

    /// Paint the results of the last call to [`Self::run`].
    pub fn paint(&mut self, canvas: &Canvas) {
        let shapes = std::mem::take(&mut self.shapes);
        let textures_delta = std::mem::take(&mut self.textures_delta);
        let clipped_primitives = self.egui_ctx.tessellate(shapes, self.pixels_per_point);
        self.painter.paint_and_update_textures(
            canvas,
            self.egui_ctx.pixels_per_point(),
            clipped_primitives,
            textures_delta,
        );
    }
}

impl Default for EguiSkia {
    fn default() -> Self {
        Self::new(1.0)
    }
}
