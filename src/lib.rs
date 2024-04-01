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

impl TestBackend {
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

    pub fn assert_screenshot_after_n_frames(
        &mut self,
        expected_file_name: &str,
        window_size: (i32, i32),
        n: usize,
        mut ui: impl FnMut(&egui::Context),
    ) {
        let mut surface =
            surfaces::raster_n32_premul(window_size).expect("Failed to create surface");
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
