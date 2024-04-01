use egui::Label;

use super::*;

#[derive(Default)]
struct EmptyApp {

}



#[test]
fn hello_world() {
    let mut backend = TestBackend::new("src/tests/expected", "src/tests/actual", |_ctx| {});
    backend.assert_screenshot_after_n_frames("hello_world.png", (300, 200), 5, |ui| {
        ui.add(Label::new("Hello World"));
    });
}