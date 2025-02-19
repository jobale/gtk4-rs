mod custom_layout;
mod custom_layout_child;
mod simple_widget;
use std::str::FromStr;

use gtk::prelude::*;
use gtk::{gdk, glib};

const COLORS: [&str; 16] = [
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "grey",
    "magenta",
    "lime",
    "yellow",
    "firebrick",
    "aqua",
    "purple",
    "tomato",
    "pink",
    "thistle",
    "maroon",
];
const TOTAL_COLORS: i32 = COLORS.len() as i32;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.custom_layout")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .default_width(600)
            .default_height(600)
            .application(app)
            .title("Custom Layout Manager")
            .build();

        let widget = simple_widget::SimpleWidget::new();
        for color in &COLORS {
            let rgba = gdk::RGBA::from_str(color).unwrap();
            let child = custom_layout_child::CustomLayoutChild::new(rgba);
            widget.add_child(&child);
        }

        window.set_child(Some(&widget));
        window.present();
    });

    application.run()
}
