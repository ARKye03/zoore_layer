use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use hyprland::dispatch;
use hyprland::{
    dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial},
    shared::{HyprData, HyprDataActive},
};
use tokio::sync::watch;

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

    border_buttons(&builder);

    render_workspaces(builder);

    window.set_child(Some(&master_center_box));
    window.set_namespace("top_bar");
    window.set_widget_name("top_bar");

    window
}

fn border_buttons(builder: &gtk::Builder) {
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
}

fn render_workspaces(builder: gtk::Builder) {
    let (tx, mut rx) = watch::channel(hyprland::data::Workspace::get_active().unwrap());

    // Spawn a new task that updates the active workspace and sends it to the watch channel
    tokio::spawn(async move {
        let mut last_active_workspace = hyprland::data::Workspace::get_active().unwrap();
        loop {
            // Update the active workspace...
            let active_workspace = hyprland::data::Workspace::get_active().unwrap();
            if active_workspace != last_active_workspace {
                tx.send(active_workspace.clone()).unwrap();
                last_active_workspace = active_workspace;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
    let workspaces_box: gtk::Box = builder
        .object("workspaces_box")
        .expect("Couldn't get GtkBox workspaces_box");

    let wicons = [" ", " ", "󰨞 ", " ", " ", "󰭹 ", " ", " ", "󰊖 ", " "];
    let arr: Vec<i32> = (1..10).collect();

    let update_workspaces = move || {
        let workspaces = hyprland::data::Workspaces::get().unwrap();

        // Clear the box before adding new buttons
        if let Some(mut child) = workspaces_box.first_child() {
            loop {
                let next = child.next_sibling();
                workspaces_box.remove(&child);
                if let Some(next_child) = next {
                    child = next_child;
                } else {
                    break;
                }
            }
        }

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
    gio::glib::MainContext::default().spawn_local(async move {
        loop {
            let _ = rx.changed().await;
            update_workspaces();
        }
    });
}
