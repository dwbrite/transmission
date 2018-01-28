extern crate ears;
extern crate cursive;

use ears::{AudioController, Sound};
use std::thread;

use cursive::Cursive;
use cursive::view::Boxable;
use cursive::views::{Button, Dialog, LinearLayout, ListView, TextArea};

fn main() {
    let mut siv = Cursive::new();

    siv.load_theme_file("src/theme.toml").unwrap();

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
            .child(Dialog::new()
                .title("Notes")
                .content(TextArea::new())
                .fixed_size((80, 24))
            )
            .child(Dialog::new()
                .title("Voicemail")
                .content(
                    ListView::new()
                        .child("G’ma - 06/12/07", create_play_button())
                        .child("Joyce - 06/13/07 6:00 p.m.", create_play_button())
                        .fixed_size((40, 24))
                )
            )
        ).fixed_width(132)
    );

    siv.run();
}

fn create_play_button() -> Button {
    Button::new("Play", |_| {
        thread::spawn(move || {
            let mut snd = Sound::new("src/test.ogg").unwrap();
            snd.play();
            while snd.is_playing() { }
        });
    })
}