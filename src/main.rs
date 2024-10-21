// use rand::Rng;
// use std::io;
use druid::{AppLauncher, Widget, WindowDesc, widget::Label};


fn build_ui() -> impl Widget<()> {
    Label::new("Hello, World!")
}
fn main() {
   let main_window= WindowDesc::new(build_ui()) ;
   AppLauncher::with_window(main_window).launch(()).expect("launch failed");
}

