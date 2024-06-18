mod utils;
mod watch;
mod widgets;

use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider};
use std::fs;
use std::process;

fn activate(application: &gtk::Application) {
    let top_bar_window = widgets::top_bar::top_bar_master::top_bar_window(application);

    top_bar_window.present()
}

fn main() {
    let application = gtk::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        activate(app);
    });
    application.connect_startup(|_| load_css());

    application.run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    let input_scss = "src/styles/master.scss";
    let output_css = "src/styles/style.css";

    process::Command::new("sassc")
        .arg(&input_scss)
        .arg(&output_css)
        .status()
        .expect("Failed to run sassc");

    let css = fs::read_to_string(output_css).expect("Failed to read CSS file");
    provider.load_from_string(&css);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    _ = watch::watch_styles();
}
