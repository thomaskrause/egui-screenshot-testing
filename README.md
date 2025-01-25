This was developed before egui hat its own UI testing library [egui_kittest](https://crates.io/crates/egui_kittest) integrated. 
There is no benefit of using this library over egui_kittest (on the contrary, egui_kittest has much more features and allows simulate UI interaction) so it has been archived.

---

# egui-screenshot-testing


Helper functions to test [egui](https://github.com/emilk/egui/) applications
using screenshots and comparing them to a saved version. 
The idea is to take an application state and render it using the
`TestBackend`. Then, you can compare a saved screenshot and compare that the
generated visuals are the same.

```rust
use egui_screenshot_testing::TestBackend;

let mut backend = TestBackend::new("src/tests/expected", "src/tests/actual", |_ctx| {
    // You could do any initialization here.
});
backend.assert_screenshot_after_n_frames("test_case_a.png", (150, 100), 5, move |ctx| {
    // Add any egui elements
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello World");
    });
});
```


The screenshots are compared to an image file that is stored in a given directory.

```plain
Cargo.toml
src/
  [...]
  tests/
    expected/
      test_case_a.png
      test_case_b.png
      [...]
```

On failure, the generated screenshot is written to a folder that contains
all actual screenshots. You can compare the images by hand or with an image
diff tool (eg. the ImageMagick `compare` tool) and decide whether you want
to update the snapshot by copying the file to the expected folder. If you
set the environment variable `EGUI_SCREENSHOT_REPLACE`, all expected files
will be replaced with the actual ones without failing the tests. This is
e.g. useful when creating an initial set of tests where no snapshot exists
yet.

Also see the `examples/` folder in the git repo for a usage example.


## 3rd party dependencies

This software depends on several 3rd party projects. In particular, it bundles
the [egui_skia crate](https://github.com/lucasmerlin/egui_skia) from Lucas
Merlin Meurer to render screenshots in headless environment in the
`src/egui_skia` subfolder. It also incorporates the [changes from Frans
Skarman](https://github.com/TheZoq2/egui_skia) that update egui_skia to work
with newer egui versions. The original license statement for egui_skia is given below.

```
MIT License

Copyright (c) 2022 Lucas Merlin Meurer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```



Additional libraries are documented in the "third-party-licenses.html" file in this folder.
