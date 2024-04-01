//! Helper functions to test [egui](https://github.com/emilk/egui/) applications
//! using screenshots and comparing them to a saved version.
//!
//! The idea is to take an application state and render it using the
//! [TestBackend]. Then, you can compare a saved screenshot an compare that the
//! generated visuals are the same.
//!
//! ```
//! use egui_screenshot_testing::TestBackend;
//! 
//! let mut backend = TestBackend::new("src/tests/expected", "src/tests/actual", |_ctx| {
//!     // You could do any initialization here.
//! });
//! backend.assert_screenshot_after_n_frames("test_case_a.png", (150, 100), 5, 
//!     move |ctx| {
//!         // Add any egui elements
//!         egui::CentralPanel::default().show(ctx, |ui| {
//!             ui.heading("Hello World");
//!        });
//!    });
//! ```
//! 
//! 
//! The screenshots are compared to an image file that is stored in a given directory (relative to the manifest file of your package).
//!
//! ```plain
//! Cargo.toml
//! src/
//!   [..]
//!   tests/
//!     expected/
//!       test_case_a.png
//!       test_case_b.png
//!       [...]
//! ```
//!
//! On failure, the generated screenshot is written to a folder that contains
//! all actual screenshots. You can compare the images by hand or with an image
//! diff tool (eg. the ImageMagick `compare` tool) and decide whether you want
//! to update the snapshot by copying the file to the expected folder.
//!
use std::path::PathBuf;

mod egui_skia;

use crate::egui_skia::EguiSkia;
use egui::Pos2;
use skia_safe::{surfaces, Surface};
use visual_hash::HasherConfig;

pub struct TestBackend {
    backend: EguiSkia,
    expected_dir: PathBuf,
    actual_dir: PathBuf,
}

/// A backend based on [egui_skia](https://github.com/lucasmerlin/egui_skia)
/// that renders the application and creates a screenshot.
impl TestBackend {
    /// Create a new test backend.
    ///
    /// * `expected_dir` - Directory in which the images files to compare
    ///   against are located. This must be relative to the directory containing
    ///   the manifest of your package.
    /// * `actual_dir` - If a test fails, the actual image should be written do
    ///   this directory. This is relative to the directory containing the manifest of your package, too.
    /// * `init_app_with_context` - A closure that will be executed once to init
    ///   the application.
    pub fn new(
        expected_dir: impl Into<PathBuf>,
        actual_dir: impl Into<PathBuf>,
        init_app_with_context: impl FnOnce(&egui::Context),
    ) -> Self {
        let backend = EguiSkia::default();
        init_app_with_context(&backend.egui_ctx);
        TestBackend {
            backend,
            expected_dir: expected_dir.into(),
            actual_dir: actual_dir.into(),
        }
    }

    fn assert_eq_screenshot(&self, expected_file_name: &str, surface: &mut Surface) {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let mut output_file_rel = self.expected_dir.clone();
        output_file_rel.push(expected_file_name);
        let output_file = manifest_dir.join(&output_file_rel);

        // Write out the screenshot to a file that is removed if test ist successful
        let mut actual_file_rel = self.actual_dir.clone();
        actual_file_rel.push(expected_file_name);

        let actual_file = manifest_dir.join(&actual_file_rel);
        std::fs::create_dir_all(&actual_file.parent().unwrap()).unwrap();

        let actual_image_skia = surface.image_snapshot();
        let skia_data = actual_image_skia
            .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
            .unwrap();
        std::fs::write(&actual_file, skia_data.as_bytes()).unwrap();

        if std::env::var("UPDATE_EXPECT").is_ok() {
            // Write current snapshot to to expected path
            let data = actual_image_skia
                .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
                .unwrap();
            std::fs::write(&output_file, data.as_bytes()).unwrap();
        }

        // Read in expected image from file
        assert!(output_file.is_file(), "Snapshot file {:#?} does not exist.", output_file);
        let expected_image = image::io::Reader::open(&output_file)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let actual_image = image::io::Reader::open(&actual_file)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        // Compare images using a visual hash
        let hasher = HasherConfig::default().to_hasher();
        let expected_hash = hasher.hash_image(&expected_image);
        let actual_hash = hasher.hash_image(&actual_image);

        let dist = actual_hash.dist(&expected_hash);
        assert!(
            dist == 0,
            "{} != {}",
            actual_file_rel.to_string_lossy(),
            output_file_rel.to_string_lossy(),
        );

        // Remove the created file
        std::fs::remove_file(actual_file).unwrap();
    }

    /// Assert that the rendered view is the same after a given number of rendered frames.
    ///
    /// * `expected_file_name` - The file name of the snapshot.
    /// * `output_size` - The dimensions of the screenshot.
    /// * `n` - Number of times the frame should be rendered before the screenshot is compared. If you use animations or other effects, this helps to render the final version.
    /// * `ui` - Closure that creates the user interface.
    ///
    /// # Panics
    ///
    /// Similar to the inbuilt `assert_` macros, this will panic if the actual and expected screenshots are not the same.
    /// It also panics if the given snapshot file to compare against does not exist.
    pub fn assert_screenshot_after_n_frames(
        &mut self,
        expected_file_name: &str,
        output_size: (i32, i32),
        n: usize,
        mut ui: impl FnMut(&egui::Context),
    ) {
        let mut surface =
            surfaces::raster_n32_premul(output_size).expect("Failed to create surface");
        let input = egui::RawInput {
            screen_rect: Some(
                [
                    Pos2::default(),
                    Pos2::new(surface.width() as f32, surface.height() as f32),
                ]
                .into(),
            ),
            ..Default::default()
        };

        for _ in 0..n {
            self.backend.run(input.clone(), &mut ui);
        }

        self.backend.paint(surface.canvas());
        self.assert_eq_screenshot(expected_file_name, &mut surface);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn non_existing_snapshot_fails() {
        let mut backend = TestBackend::new("src/tests/expected", "src/tests/actual", |_ctx| {
            // You could do any initialization here.
        });
        backend.assert_screenshot_after_n_frames("should_not_exist.png", (150, 100),
            5, 
            move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Hello World");
           });
       });
    
    }
}
