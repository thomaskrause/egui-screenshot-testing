use egui::{Label, Visuals, Widget};

#[derive(Default)]
struct HelloApp {
    light_theme: bool,
    counter: u128,
}

impl HelloApp {
    fn init(&self, ctx: &egui::Context) {
        if self.light_theme {
            ctx.set_visuals(Visuals::light());
        } else {
            ctx.set_visuals(Visuals::dark());
        }
    }

    /// Render the context without access to the eframe::Frame.
    fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");
            if ui.button("Click me").clicked() {
                self.counter += 1;
            };
            ui.separator();
            Label::new(format!("Clicked {} times", self.counter))
                .truncate(true)
                .ui(ui);
            ui.separator();
        });
    }
}

impl eframe::App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // We do the actual rendering in a separate function that does not take
        // the eframe::Frame as argument to make it easier to use it in the
        // screenshot tests.
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
        Box::new(|ctx| {
            let app = HelloApp::default();
            app.init(&ctx.egui_ctx);
            Box::new(app)
        }),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::HelloApp;
    use egui::Visuals;
    use egui_screenshot_testing::TestBackend;

    /// Simple test that renders the application with the test backend and
    /// compares the screenshot with the stored one.
    #[test]
    fn test_initial_screen() {
        // Create you application like normal.
        let mut app = HelloApp::default();

        // Define the backend with two directories where all the screenshot files are located.
        let mut backend = TestBackend::new("examples/expected", "examples/actual", |ctx| {
            app.init(ctx);
        });

        backend.assert_screenshot_after_n_frames(
            "hello_world_initial.png",
            (150, 100),
            5,
            move |ctx| {
                app.render(ctx);
            },
        );
    }

    /// This crate does not allow to emulate any input to egui, but it can
    /// modify the application state and check that the result renders
    /// correctly and that the number is truncated.
    #[test]
    fn test_large_number() {
        let mut app = HelloApp::default();
        // Use a very large number and check that the rendering still works.
        app.counter = 100_000_000_000_000;

        let mut backend = TestBackend::new("examples/expected", "examples/actual", |ctx| {
            app.init(ctx);
        });

        backend.assert_screenshot_after_n_frames(
            "hello_world_large_number.png",
            (150, 100),
            5,
            move |ctx| {
                app.render(ctx);
            },
        );
    }

    /// Use the init closure to set the theme once.
    #[test]
    fn test_light_theme() {
        let mut app = HelloApp::default();

        let mut backend = TestBackend::new("examples/expected", "examples/actual", |ctx| {
            ctx.set_visuals(Visuals::light());
        });

        backend.assert_screenshot_after_n_frames(
            "hello_world_light_theme.png",
            (150, 100),
            5,
            move |ctx| {
                app.render(ctx);
            },
        );
    }
}
