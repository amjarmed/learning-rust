// use rand::Rng;
// use std::io;
use druid::{AppLauncher, Widget, WindowDesc, widget::Label};


fn build_ui() -> impl Widget<()> {
    Label::new("Hello, World!")
}
fn App() {
   let main_window= WindowDesc::new(build_ui()) ;
   AppLauncher::with_window(main_window).launch(()).expect("launch failed");
}

/* 
1. Choose a GUI Framework

- Tauri
- Druid
- Iced
- Slint

2. System-Level Libraries

- Tokio/Async-Std: For async networking.
- Serde: For serializing/deserializing data.
- Filesystem libraries: Such as std::fs and walkdir.
- Cross-platform libraries: Such as dirs for - platform-specific directories.


3. Cross-Platform Development
Rust is inherently cross-platform, so if you aim for Windows, macOS, and Linux, Rust’s cross-compilation capability makes it a suitable language for desktop apps that need to work on different operating systems.

4. Packaging and Distribution

- Tauri has built-in packaging and signing features, similar to Electron apps, so you can easily create installers.

Cargo-bundle: For other GUI frameworks, the cargo-bundle crate can be used to create .exe, .app, or .deb bundles for your app.


*/
