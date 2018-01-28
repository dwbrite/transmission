extern crate cursive;
extern crate ears;
extern crate gag;

use std::thread;

use cursive::Cursive;
use cursive::traits::*;
use cursive::view::Boxable;
use cursive::views::{Button, Dialog, LinearLayout, ListView, TextArea, TextView};

use ears::{AudioController, Sound};

use gag::Gag;
use std::cell::Cell;

struct StateManager <'a>{
    state_number: Cell<usize>,
    dialogue: &'a[&'a str],
}

impl<'a> StateManager <'a> {
    fn state_number(&self) -> usize {
        self.state_number.get()
    }
}

fn main() {
    let state_manager = StateManager {
        state_number: Cell::new(0),
        dialogue: &["Finally...back from work. Your wife isn’t home yet...again.",
            "She’ll probably say, “she’s working late”.",
            "Checking your phone, you notice a voicemail from her.",
            "No surprise there. You might want to get to the bottom of this."]
    };

    let _gag_stderr = Gag::stderr().unwrap();

    let mut siv = Cursive::new();

    siv.load_theme_file("src/theme.toml").unwrap();

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(LinearLayout::horizontal().with_id("desktop"))
                .child(Dialog::around(TextView::new(state_manager.dialogue[0]).with_id("dialogue")))
                .child(Button::new("Continue", move |s| continue_game(s, &state_manager)
        ).fixed_width(132)
    )));

    siv.run();
}

fn create_play_button() -> Button {
    Button::new("Play", |_| {
        thread::spawn(move || {
            let mut sound = Sound::new("src/test.ogg").unwrap();
            sound.play();
            while sound.is_playing() {}
        });
    })
}

fn continue_game(s: &mut Cursive, sm: &StateManager) {
    sm.state_number.set(sm.state_number() + 1);

    if sm.state_number() == 1 {
        enable_voicemail(s);
    }

    let next_dialogue = sm.dialogue[sm.state_number()];

    s.call_on_id("dialogue", |view: &mut TextView| {
        view.set_content(next_dialogue);
    });


    add_voicemail(s, "Hello!", ":(");
}

fn enable_voicemail(s: &mut Cursive) {
    s.call_on_id("desktop", |view: &mut LinearLayout| {
        view.add_child(Dialog::new()
            .title("Voicemail")
            .content(
                ListView::new()
                    .fixed_size((40, 24))
                    .with_id("voicemail")
            )
        );
    });
}

fn enable_notes(s: &mut Cursive) {
    s.call_on_id("desktop", |view: &mut LinearLayout| {
        view.add_child(Dialog::new()
            .title("Notes")
            .content(TextArea::new())
            .fixed_size((80, 24))
        );
    });
}

fn add_voicemail(s: &mut Cursive, title: &str, path: &str) {
    s.call_on_id("voicemail", |view: &mut ListView| {
        view.add_child(title, create_play_button());
    });
}