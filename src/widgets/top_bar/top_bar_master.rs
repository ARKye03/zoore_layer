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

    let builder = gtk::Builder::from_file("src/widgets/top_bar/ui/top_bar.ui");
    let master_center_box: gtk::CenterBox = builder
        .object("bar_CenterBox")
        .expect("Couldn't get GtkCenterBox");

    let app_launcher_button: gtk::Button = builder
        .object("app_launcher_button")
        .expect("Couldn't get GtkButton app_launcher_button");

    let app_launcher_button_icon: gtk::Image = builder
        .object("app_launcher_button_icon")
        .expect("Couldn't get GtkImage app_launcher_button_icon");
    app_launcher_button_icon.set_pixel_size(25);
    app_launcher_button_icon.set_from_file(Some("assets/applauncher.svg"));

    button.set_child(Some(&icon));
    window.set_child(Some(&master_center_box));
    window.set_namespace("top_bar");
    window.set_widget_name("top_bar");

    window
}

/* import Hyprland from "resource:///com/github/Aylur/ags/service/hyprland.js";
import Widget from "resource:///com/github/Aylur/ags/widget.js";
import { execAsync } from "resource:///com/github/Aylur/ags/utils.js";

export const Workspaces = () =>
  Widget.Box({
    className: "workspaces",
    setup: (self) => {
      const Wicons = [
        "",
        " ",
        " ",
        "󰨞 ",
        " ",
        " ",
        "󰭹 ",
        " ",
        " ",
        "󰊖 ",
        " ",
      ];
      const arr = Array.from({ length: 10 }, (_, i) => i + 1);

      const updateWorkspaces = () => {
        self.children = arr.map((i) => {
          let className = ""; // default value

          if (Hyprland.active.workspace.id === i) {
            className = "focused";
          } else if (
            Hyprland.workspaces.some((ws) => ws.id === i && ws.windows > 0)
          ) {
            className = "work";
          }

          return Widget.Button({
            onClicked: () =>
              execAsync(`/usr/bin/hyprctl dispatch workspace ${i}`),
            child: Widget.Label(`${Wicons[i]}`),
            className: className,
          });
        });
      };

      self.hook(Hyprland.active.workspace, updateWorkspaces, "changed");
      updateWorkspaces();
    },
  });
 */
