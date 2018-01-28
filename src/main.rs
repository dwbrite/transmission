extern crate ears;
extern crate cursive;

use ears::{Sound, AudioController};
use std::time::Duration;
use std::thread::sleep;
use std::thread;

use cursive::Cursive;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::align::*;
use cursive::view::Boxable;
use cursive::view::SizeConstraint;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ListView, TextArea, TextView};

fn main() {


    // Read some long text from a file.
    let content = include_str!("../story.md");

    let mut siv = Cursive::new();

    siv.load_theme_file("src/theme.toml").unwrap();

    // The text is too long to fit on a line, so the view will wrap lines,
    // and will adapt to the terminal size.

    let text = "This is a very simple example of linear layout. Two views \
                are present, a short title above, and this text. The text \
                has a fixed width, and the title is centered horizontally.";

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