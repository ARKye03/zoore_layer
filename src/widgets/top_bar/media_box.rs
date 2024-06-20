use gtk::prelude::*;
use mpris::PlayerFinder;

pub fn media_box(builder: &gtk::Builder) {
    let media_button: gtk::Button = builder
        .object("media_button")
        .expect("Couldn't get media_button");

    let players = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_all()
        .expect("Could not find any player");

    let player = players
        .into_iter()
        .find(|p| p.identity().contains("MPD"))
        .expect("Could not find MPD player");

    let metadata = player
        .get_metadata()
        .expect("Could not get metadata for player");

    if let Some(title) = metadata.title() {
        media_button.set_label(title);
    } else {
        media_button.set_label("No title");
    }
}
