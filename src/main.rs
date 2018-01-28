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

struct StateManager {
    state_number: Cell<usize>,
    states: Vec<Box<Fn(&mut Cursive)>>
}

impl StateManager {
    fn next_state(&self, s: &mut Cursive) {
        (self.states[self.state_number.get()])(s);
        self.state_number.set(self.state_number.get() + 1);
    }
}

fn main() {
    let state_manager = StateManager {
        state_number: Cell::new(0),
        states: vec![Box::new(|s: &mut Cursive| update_dialogue(s, "[Wednesday 6:30 p.m.]")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "Finally...back from work. Your wife isn’t home yet...again.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "She’ll probably say, “she’s working late”.")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "Checking your phone, you notice a voicemail from her.");
                enable_voicemail(s);
                add_voicemail(s, "Joyce - 05/03/2007 6:30 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "No surprise there. You might want to get to the bottom of this.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[Thursday 5:30 a.m.]")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "You wake up early to install an wiretapping app. It will send all her calls directly to your voicemail. You’ve done your research, clearly.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[3:23 p.m.]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "At work, you notice a new voicemail.");
                add_voicemail(s, "Joyce - 05/04/2007 3:02 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "It seems to have cut off some of the audio. Must be a software issue.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "You get home that night, but don’t mention anything to your wife.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[Friday 10:07 a.m.]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "Another voicemail.");
                add_voicemail(s, "Joyce - 05/05/2007 10:00 a.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "So that’s what she’s up to. Who could that be?")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[Monday 3:00 p.m.]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "Nothing interesting happened over the weekend. You’ve finally got another voicemail.");
                add_voicemail(s, "Joyce - 05/08/2007 2:57 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "Another voicemail enters your inbox.");
                add_voicemail(s, "Joyce - 05/08/2007 3:01 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "Shit.")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "Maybe you should start keeping track of this.");
                enable_notes(s);
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[8:30 p.m.]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "You’re watching TV when you hear your phone buzz.");
                add_voicemail(s, "Joyce - 05/08/2007 8:30 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "Ouch. Maybe it’ll be better to go to bed before she gets home.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[Wednesday]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "*bzzzt*");
                add_voicemail(s, "Joyce - 05/10/2007 2:29 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "...that is *not* something you wanted to hear.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[Friday 11:30 a.m.]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "Time for lunch...it looks like you have a voicemail from your friend, Dustin.");
                add_voicemail(s, "Dusty - 05/12/2007 11:25 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "Pfft, a little late if you ask me...Dusty’s always up in everyone’s business anyways.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "You need to talk to your wife tonight.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[5:05 p.m.]")),
            Box::new(|s: &mut Cursive| {
                update_dialogue(s, "*bzzzt*");
                add_voicemail(s, "Joyce - 05/12/2007 5:05 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[7:00 p.m.]")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "She’s not home yet. She didn't even bother to call.")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "[7:30 p.m.]")),
            Box::new(|s: &mut Cursive| {
               update_dialogue(s, "*bzzzt*");
               add_voicemail(s, "Joyce - 05/12/2007 7:30 p.m.", "src/test.ogg");
            }),
            Box::new(|s: &mut Cursive| update_dialogue(s, "...")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "dialogue")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "dialogue")),
            Box::new(|s: &mut Cursive| update_dialogue(s, "dialogue")),
        ]
    };

    let _gag_stderr = Gag::stderr().unwrap();

    let mut siv = Cursive::new();

    siv.load_theme_file("src/theme.toml").unwrap();

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(LinearLayout::horizontal().with_id("desktop"))
                .child(Dialog::around(TextView::new("Click continue to start!").with_id("dialogue")))
                .child(Button::new("Continue", move |s| continue_game(s, &state_manager)
        ).fixed_width(132)
    )));

    siv.run();
}

fn create_play_button(path: &'static str) -> Button {

    Button::new("Play",  move |_| {
        thread::spawn(move || {
            let mut sound = Sound::new(path).unwrap();
            sound.play();
            while sound.is_playing() {}
        });
    })
}

fn continue_game(s: &mut Cursive, sm: &StateManager) {
    sm.next_state(s);
}

fn update_dialogue(s: &mut Cursive, dialogue: &str) {
    s.call_on_id("dialogue", |view: &mut TextView| {
        view.set_content(dialogue);
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

// good functions :)

fn enable_voicemail(s: &mut Cursive) {
    s.call_on_id("desktop", |view: &mut LinearLayout| {
        view.add_child(Dialog::new()
            .title("Voicemail")
            .content(ListView::new()
            .with_id("voicemail")
            .fixed_size((40, 24))
        ));
    });
}

// bad functions :(

fn add_voicemail(s: &mut Cursive, title: &str, path: &'static str) {
    s.call_on_id("voicemail", move |view: &mut ListView| {
        view.add_child(title, create_play_button(path));
    });
}