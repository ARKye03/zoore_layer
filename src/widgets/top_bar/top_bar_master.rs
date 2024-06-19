use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub fn top_bar_window(application: &gtk::Application) -> ApplicationWindow {
    let window = gtk::ApplicationWindow::new(application);

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.auto_exclusive_zone_enable();

    window.set_margin(Edge::Left, 10);
    window.set_margin(Edge::Right, 10);
    window.set_margin(Edge::Bottom, 20);

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }
    let builder = gtk::Builder::from_file("src/widgets/top_bar/ui/top_bar.ui");

    let master_center_box: gtk::CenterBox = builder
        .object("bar_CenterBox")
        .expect("Couldn't get GtkCenterBox");

    super::border_buttons::border_buttons(&builder);

    super::workspace_renderer::render_workspaces(builder);

    window.set_child(Some(&master_center_box));
    window.set_namespace("top_bar");
    window.set_widget_name("top_bar");

    window
}
