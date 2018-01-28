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
    stateNumber: Cell<usize>,
    dialogue: &'a[&'a str],
}

fn main() {
    //let dia = ;

    let state_manager = StateManager {
        stateNumber: Cell::new(0),
        dialogue: &["Finally...back from work. Your wife isn’t home yet...again.", "She’ll probably say, “she’s working late”.", "Checking your phone, you notice a voicemail from her."]
    };

    let _gag_stderr = Gag::stderr().unwrap();

    let mut siv = Cursive::new();

    siv.load_theme_file("src/theme.toml").unwrap();

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
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
                )
                .child(Dialog::around(TextView::new("\n\n\n").with_id("dialogue")))
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
    //let _s2 = sm.clone();

    sm.stateNumber.set(sm.stateNumber() + 1);
    println!("state: {}", sm.stateNumber());
    //let &mut idk = (&mut sm.stateNumber);
    //*idk += 1;
    //*step += 1;

    /*s.call_on_id("dialogue", |view: &mut TextView| {
        let content = dialogue[*step];
        view.set_content(content);
    });*/
}

impl<'a> Clone for StateManager<'a> {
    fn clone(&self) -> Self {
        self.stateNumber.set(self.stateNumber.get() + 1);
        StateManager {
            stateNumber: self.stateNumber.clone(),
            dialogue: self.dialogue.clone(),
        }
    }
}

impl<'a> StateManager <'a> {
    fn stateNumber(&self) -> usize {
        self.stateNumber.get()
    }
}