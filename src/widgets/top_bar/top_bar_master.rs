use crate::utils::exec_async::exec_async;
use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use hyprland::dispatch;
use hyprland::dispatch::DispatchType::*;
use hyprland::dispatch::WorkspaceIdentifier;
use hyprland::dispatch::{
    Corner, Dispatch, DispatchType, FullscreenType, WorkspaceIdentifierWithSpecial,
};
use hyprland::shared::HyprData;
use hyprland::shared::HyprDataActive;

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
        std::process::Command::new("rofi")
            .arg("-show")
            .arg("drun")
            .spawn()
            .expect("Failed to execute process");
    });

    let sys_button: gtk::Button = builder
        .object("sys_button")
        .expect("Couldn't get GtkButton sys_button");
    sys_button.connect_clicked(|_| {
        std::process::Command::new("/home/archkye/.config/rofi/powermenu/type-4/powermenu.sh")
            .spawn()
            .expect("Failed to execute process");
    });

    let app_launcher_button_icon: gtk::Image = builder
        .object("app_launcher_button_icon")
        .expect("Couldn't get GtkImage app_launcher_button_icon");
    app_launcher_button_icon.set_pixel_size(25);
    app_launcher_button_icon.set_from_file(Some("assets/applauncher.svg"));

    let workspaces_box: gtk::Box = builder
        .object("workspaces_box")
        .expect("Couldn't get GtkBox workspaces_box");

    let wicons = [" ", " ", "󰨞 ", " ", " ", "󰭹 ", " ", " ", "󰊖 ", " "];
    let arr: Vec<i32> = (1..10).collect();

    let update_workspaces = || {
        let workspaces = hyprland::data::Workspaces::get().unwrap();

        for i in &arr {
            let mut class_name = "";
            match hyprland::data::Workspace::get_active() {
                Ok(active_workspace) => {
                    if active_workspace.id == *i {
                        class_name = "focused";
                    } else if workspaces.iter().any(|ws| ws.id == *i && ws.windows > 0) {
                        class_name = "work";
                    }
                }
                Err(e) => {
                    eprintln!("Error getting active workspace: {:?}", e);
                }
            }
            let w_button = gtk::Button::new();
            w_button.set_label(wicons[*i as usize - 1]);
            w_button.set_css_classes(&[class_name]);
            let i_clone = *i; // Clone `i` here
            w_button.connect_clicked(move |_| {
                // Move the cloned `i` into the closure
                _ = dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(i_clone));
            });
            workspaces_box.append(&w_button)
        }
    };
    update_workspaces();

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
