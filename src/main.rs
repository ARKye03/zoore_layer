use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::fs;
use std::process;

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Top);

    // Push other windows out of the way
    window.auto_exclusive_zone_enable();

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 20);
    window.set_margin(Edge::Right, 20);
    window.set_margin(Edge::Top, 10);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // Set up a widget
    let button = gtk::Button::new();
    let icon = gtk::Image::new();
    icon.set_from_file(Some("src/send.svg"));
    button.connect_clicked(|b| {
        b.set_label("label");
        println!("Hello")
    });

    let builder = gtk::Builder::from_file("src/ui/topbat.ui");
    let master_center_box: gtk::CenterBox = builder
        .object("GtkCenterBox")
        .expect("Couldn't get GtkCenterBox");

    button.set_child(Some(&icon));
    window.set_child(Some(&master_center_box));
    window.set_visible(true);
    window.present()
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
}
