use gtk::{prelude::*, Label};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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
    let label = gtk::Label::new(Some("Haaaaa"));
    let button = gtk::Button::new();
    let icon = gtk::Image::new();
    icon.set_from_file(Some("src/send.svg"));
    button.connect_clicked(|b| {
        b.set_label("label");
        println!("Hello")
    });

    let master_center_box = gtk::CenterBox::new();
    master_center_box.set_start_widget(Some(&button));
    master_center_box.set_center_widget(Some(&label));
    master_center_box.set_end_widget(Some(&Label::new(Some("XXDDDD"))));

    button.set_child(Some(&icon));
    window.set_child(Some(&master_center_box));
    window.present()
}

fn main() {
    let application = gtk::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}
