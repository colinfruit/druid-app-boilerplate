use druid::widget::prelude::*;
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{AppLauncher, Size, WindowDesc};

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .show_titlebar(false)
        .resizable(false)
        .with_min_size(Size::ZERO)
        // TODO: there's definitely a better way of doing this.
        .window_size((360., 648.))
        .title("Druid app boilerplate");

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<()> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .must_fill_main_axis(true)
        .with_child(Label::new("Hello Pinephone!"))
}
