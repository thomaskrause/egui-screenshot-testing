#[derive(Default)]
struct HelloApp {
    counter: u32,
}

impl HelloApp {
    fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");
            if ui.button("Click me").clicked() {
                self.counter += 1;
            };
            ui.separator();
            ui.label(format!("Clicked {} times", self.counter));
            ui.separator();
        });
    }
}

impl eframe::App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([150.0, 100.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hello World",
        options,
        Box::new(|_| Box::<HelloApp>::default()),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::HelloApp;
    use egui_screenshot_testing::TestBackend;

    #[test]
    fn test_initial_screen() {
        let mut app = HelloApp::default();
        let mut backend = TestBackend::new("examples/expected", "examples/actual", |_ctx| {});

        backend.assert_screenshot_after_n_frames(
            "initial_hello_world.png",
            (150, 100),
            5,
            move |ctx| {
                app.render(ctx);
            },
        );
    }
}
