# egui-screenshot-testing

[![Documentation](https://docs.rs/egui-screenshot-testing/badge.svg)](https://docs.rs/egui-screenshot-testing)

Helper functions to test [egui](https://github.com/emilk/egui/) applications
using screenshots and comparing them to a saved version. See the `examples/`
folder in the git repo for a usage example.


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
