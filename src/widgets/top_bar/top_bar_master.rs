use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::utils;

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

    let app_launcher_button: gtk::Button = builder
        .object("app_launcher_button")
        .expect("Couldn't get GtkButton app_launcher_button");
    app_launcher_button.connect_clicked(|_| {
        println!("clicked");
        _ = utils::exec_async::exec_async("/usr/bin/thunar");
    });

    let app_launcher_button_icon: gtk::Image = builder
        .object("app_launcher_button_icon")
        .expect("Couldn't get GtkImage app_launcher_button_icon");
    app_launcher_button_icon.set_pixel_size(25);
    app_launcher_button_icon.set_from_file(Some("assets/applauncher.svg"));

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
