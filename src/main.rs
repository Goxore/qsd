use id3::{Error, ErrorKind, Tag, TagLike, Version};
use gtk::{glib, prelude::*};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{process::Command, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(build_ui);

    application.run();

    Ok(())
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    // window.set_default_size(350, 70);

    let title_text = gtk::Label::new(Some("title"));

    let entry_url = gtk::Entry::new();
    entry_url.set_text("enter url");

    match ClipboardContext::new(){
        Ok(mut res) => entry_url.set_text(&res.get_contents().unwrap_or(String::from("insert url"))),
        Err(..) => entry_url.set_text("insert url"),
    }

    let entry_artist = gtk::Entry::new();
    entry_artist.set_text("input artist");

    let entry_album = gtk::Entry::new();
    entry_album.set_text("input album");

    let entry_name = gtk::Entry::new();
    entry_name.set_text("input song name");

    let start_button = gtk::Button::new();
    start_button.set_label("click");

    start_button.connect_clicked(glib::clone!(@weak window, @weak start_button,@weak entry_url, @weak entry_artist, @weak entry_album, @weak entry_name => move |_| {
        start_button.set_label("downloading...");
        start_button.set_opacity(0.3);
        start_button.set_sensitive(false);
        entry_url.set_sensitive(false);
        entry_artist.set_sensitive(false);
        entry_album.set_sensitive(false);
        entry_name.set_sensitive(false);

        change_id3data(
            entry_url.text().as_str(),
            entry_artist.text().as_str(),
            entry_album.text().as_str(),
            entry_name.text().as_str()
        ).unwrap();


        let _ = &window.close();
    }));

    let gtkbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    gtkbox.add(&title_text);
    gtkbox.add(&entry_url);
    gtkbox.add(&entry_artist);
    gtkbox.add(&entry_album);
    gtkbox.add(&entry_name);
    gtkbox.add(&start_button);

    window.add(&gtkbox);

    window.show_all();
}

fn change_id3data(download_url: &str,new_artist_name: &str, new_album_name: &str, new_song_name: &str) 
    -> Result<(), Box<dyn std::error::Error>> 
{

    let song_path_name = format!("{}.mp3", new_song_name);
    let song_full_path = format!("{}{}{}", dirs::audio_dir().unwrap_or(PathBuf::from(".")).to_string_lossy(), std::path::MAIN_SEPARATOR, song_path_name);

    let mut command = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-quality")
        .arg("0")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--prefer-ffmpeg")
        .arg("--prefer-ffmpeg")
        .arg(download_url)
        .arg("-o")
        .arg(&song_full_path)
        .spawn()
        .expect("command failed to start");

    let _ = &command.wait();

    println!("downloaded");

    let mut tag = match Tag::read_from_path(&song_full_path) {
        Ok(tag) => tag,
        Err(Error {
            kind: ErrorKind::NoTag,
            ..
        }) => Tag::new(),
        Err(err) => return Err(Box::new(err)),
    };

    tag.set_album(new_album_name);
    tag.set_artist(new_artist_name);

    tag.write_to_path(&song_full_path, Version::Id3v24)?;

    println!("{}", tag.artist().unwrap());

    Ok(())
}
