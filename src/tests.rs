use egui::{Button, Label};

use super::*;

#[derive(Default)]
struct EmptyApp {}

#[test]
fn hello_world() {
    let mut backend = TestBackend::new("src/tests/expected", "src/tests/actual", |_ctx| {});
    backend.assert_screenshot_after_n_frames("hello_world.png", (150, 100), 5, |ui| {
        ui.add(Label::new("Hello World"));
        ui.add(Button::new("Click me"));
    });
}
