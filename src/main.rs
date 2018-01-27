extern crate ears;
extern crate cursive;

use ears::{Sound, AudioController};
use std::time::Duration;
use std::thread::sleep;

use cursive::Cursive;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::align::*;
use cursive::view::Boxable;
use cursive::view::SizeConstraint;
use cursive::views::{Dialog, Panel, TextView, TextArea};


fn main() {
    // let mut snd = Sound::new("src/test.ogg").unwrap();
    // snd.play();

    // Read some long text from a file.
    let content = include_str!("../story.md");

    let mut siv = Cursive::new();

    // The text is too long to fit on a line, so the view will wrap lines,
    // and will adapt to the terminal size.

    siv.add_layer(
        Dialog::new()
            .title("Notes")
            .content(TextArea::new())
            .fixed_size((80, 24))
    );


    siv.run();
}