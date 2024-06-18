use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub fn top_bar_window(application: &gtk::Application) -> ApplicationWindow {
    let window = gtk::ApplicationWindow::new(application);
    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Top);

    // Push other windows out of the way
    window.auto_exclusive_zone_enable();

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 10);
    window.set_margin(Edge::Right, 10);
    window.set_margin(Edge::Bottom, 25);

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

    let builder = gtk::Builder::from_file("src/top_bar/ui/top_bar.ui");
    let master_center_box: gtk::CenterBox = builder
        .object("bar_CenterBox")
        .expect("Couldn't get GtkCenterBox");

    button.set_child(Some(&icon));
    window.set_child(Some(&master_center_box));
    window.set_namespace("top_bar");
    window.set_widget_name("top_bar");

    window
}
