//! Helper functions to test [egui](https://github.com/emilk/egui/) applications
//! using screenshots and comparing them to a saved version.
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
    ///   against are located.
    /// * `actual_dir` - If a test fails, the actual image should be written do
    ///   this directory.
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

    fn assert_eq_screenshot(
        &self,
        expected_file_name: &str,
        surface: &mut Surface,
        replace_if_not_equal: bool,
    ) {
        let output_file = self.expected_dir.join(expected_file_name);

        // Write out the screenshot to a file that is removed if test ist successful
        let actual_file = self.actual_dir.join(expected_file_name);

        std::fs::create_dir_all(actual_file.parent().unwrap()).unwrap();

        let actual_image_skia = surface.image_snapshot();
        let skia_data = actual_image_skia
            .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
            .unwrap();
        std::fs::write(&actual_file, skia_data.as_bytes()).unwrap();

        if replace_if_not_equal {
            // Write current snapshot to to expected path
            let data = actual_image_skia
                .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
                .unwrap();
            std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();
            std::fs::write(&output_file, data.as_bytes()).unwrap();
        }

        // Read in expected image from file
        assert!(
            output_file.is_file(),
            "Snapshot file {:#?} does not exist.",
            output_file
        );
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
            actual_file.to_string_lossy(),
            output_file.to_string_lossy(),
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
        let replace_if_not_equal = std::env::var("EGUI_SCREENSHOT_REPLACE").is_ok();
        self.assert_eq_screenshot(expected_file_name, &mut surface, replace_if_not_equal);
    }
}

#[cfg(test)]
mod tests {

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn readme_example() {
        let mut backend = TestBackend::new("src/tests/expected", "src/tests/actual", |_ctx| {
            // You could do any initialization here.
        });
        backend.assert_screenshot_after_n_frames("test_case_a.png", (150, 100), 5, move |ctx| {
            // Add any egui elements
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Hello World");
            });
        });
    }

    #[test]
    #[should_panic]
    fn non_existing_snapshot_fails() {
        let out_dir = tempdir().unwrap();

        temp_env::with_var_unset("EGUI_SCREENSHOT_REPLACE", || {
            let mut backend = TestBackend::new(
                "src/tests/expected",
                out_dir.path().join("actual"),
                |_ctx| {},
            );
            backend.assert_screenshot_after_n_frames(
                "should_not_exist.png",
                (150, 100),
                5,
                move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Hello World");
                    });
                },
            );
        });
    }

    #[test]
    fn replace_env_variable() {
        let out_dir = tempdir().unwrap();

        let expected = out_dir.path().join("expected");
        let actual = out_dir.path().join("actual");

        temp_env::with_var("EGUI_SCREENSHOT_REPLACE", Some("1"), || {
            let mut backend = TestBackend::new(&expected, &actual, |_ctx| {});
            backend.assert_screenshot_after_n_frames(
                "will_be_created_by_env.png",
                (150, 100),
                5,
                move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Hello World");
                    });
                },
            );
        });

        assert_eq!(true, expected.join("will_be_created_by_env.png").is_file());
        assert_eq!(false, actual.join("will_be_created_by_env.png").exists());
    }
}
