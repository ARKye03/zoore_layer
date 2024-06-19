use gtk::prelude::*;

pub fn border_buttons(builder: gtk::Builder) {
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
