use gtk::prelude::*;
use hyprland::dispatch;
use hyprland::{
    dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial},
    shared::{HyprData, HyprDataActive},
};
use tokio::sync::watch;

pub fn render_workspaces(builder: gtk::Builder) {
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

    let wicons = [
        " ", " ", " ", "󰨞 ", " ", " ", "󰭹 ", " ", " ", "󰊖 ", " ",
    ];
    let arr: Vec<i32> = (1..10).collect();

    // Create the buttons once and store them in a Vec
    let mut buttons: Vec<gtk::Button> = Vec::new();
    for i in &arr {
        let w_button = gtk::Button::new();
        // Skip the first icon by starting from 1
        w_button.set_label(wicons[*i as usize]);
        let i_clone = *i; // Clone `i` here
        w_button.connect_clicked(move |_| {
            // Move the cloned `i` into the closure
            _ = dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(i_clone));
        });
        workspaces_box.append(&w_button);
        buttons.push(w_button);
    }

    let update_workspaces = move || {
        let workspaces = hyprland::data::Workspaces::get().unwrap();

        for (i, button) in buttons.iter().enumerate() {
            let mut class_name = "";
            match hyprland::data::Workspace::get_active() {
                Ok(active_workspace) => {
                    // Subtract 1 from the workspace ID to get the correct index
                    if active_workspace.id == (i as i32 + 1) {
                        class_name = "focused";
                    } else if workspaces
                        .iter()
                        .any(|ws| ws.id == (i as i32 + 1) && ws.windows > 0)
                    {
                        class_name = "work";
                    }
                }
                Err(e) => {
                    eprintln!("Error getting active workspace: {:?}", e);
                }
            }
            button.set_css_classes(&[class_name, "workspace_button"]);
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
