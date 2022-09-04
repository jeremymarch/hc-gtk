use gtk::prelude::*;
use gtk::{Application};
use gtk::Button;
use gtk::StyleContext;
use gtk::CssProvider;
use gtk::gdk::Display;
use adw::{ApplicationWindow, HeaderBar};
use gtk::EventControllerKey;
use gtk::Inhibit;
use gtk::gdk::Key;

use rustunicodetests::*;
use form_selector::*;
use hoplite_verbs_rs::*;

use std::sync::{Arc, Mutex};

const APP_ID: &str = "com.philolog.hc-gtk";

fn build_ui(app: &Application) {
    let changed_form_tv_orig: gtk::TextView = gtk::TextView::new();
    changed_form_tv_orig.set_wrap_mode(gtk::WrapMode::Word);
    changed_form_tv_orig.set_cursor_visible(false);
    changed_form_tv_orig.set_editable(false);
    changed_form_tv_orig.set_justification(gtk::Justification::Center);
    let margin = 10;
    changed_form_tv_orig.set_left_margin(margin);
    changed_form_tv_orig.set_top_margin(margin);
    changed_form_tv_orig.set_right_margin(margin);
    changed_form_tv_orig.set_bottom_margin(margin);

    let changed_form_scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .vexpand(true)
        .child(&changed_form_tv_orig)
        .build();

    let starting_form_tv_orig: gtk::TextView = gtk::TextView::new();
    starting_form_tv_orig.set_wrap_mode(gtk::WrapMode::Word);
    starting_form_tv_orig.set_cursor_visible(false);
    starting_form_tv_orig.set_justification(gtk::Justification::Center);
    starting_form_tv_orig.set_left_margin(margin);
    starting_form_tv_orig.set_top_margin(margin);
    starting_form_tv_orig.set_right_margin(margin);
    starting_form_tv_orig.set_bottom_margin(margin);
    starting_form_tv_orig.set_editable(false);
        
    let starting_form_scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .vexpand(true)
        .child(&starting_form_tv_orig)
        .build();

    let change_label = gtk::Label::new(Some(""));
    //label.set_markup("First <b>Singular</b> Present <b>Active</b> Indicative");

    let correct_label = gtk::Label::new(Some(""));

    let button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 4);
    vbox.set_homogeneous(false);
    vbox.append(&HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Hoplite Challenge", ""))
        .build(),
    );
    vbox.append(&starting_form_scrolled_window);
    vbox.append(&change_label);
    vbox.append(&changed_form_scrolled_window);
    vbox.append(&correct_label);
    vbox.append(&button);

    let chooser = Arc::new(Mutex::new(init_random_form_chooser("../hoplite_verbs_rs/testdata/pp.txt", 20)));
    if let Ok(mut ch) = chooser.lock() {
        ch.set_reps_per_verb(4);
    }
    let starting_form_tv = starting_form_tv_orig.clone();
    let changed_form_tv = changed_form_tv_orig.clone();
    button.connect_clicked(move |button| {
        if let Ok(mut ch) = chooser.lock() {
            if ch.history.len() == 0 {
                _ = ch.next_form(None);
            }

            if button.label().unwrap() == "Submit" {
                changed_form_tv.set_editable(false);
                changed_form_tv.set_cursor_visible(false);
                let answer = changed_form_tv.buffer().text(&changed_form_tv.buffer().start_iter(), &changed_form_tv.buffer().end_iter(), false);
                let prev_vf = &ch.history[ch.history.len() - 1]; //call here before calling next_form()
                let answer_correct = prev_vf.get_form(false).unwrap().last().unwrap().form.to_string();
                
                if let Ok(vf) = ch.next_form(Some(&answer)) {
                    let is_correct_option = vf.1;
                    if let Some(is_correct) = is_correct_option {
                        if is_correct {
                            println!("correct");
                            correct_label.set_markup("<span foreground=\"green\">correct</span>");
                        }
                        else {
                            println!("incorrect");
                            correct_label.set_markup(format!("<span foreground=\"red\">incorrect: {}</span>", answer_correct).as_str());

                            //new verb!
                            // ch.verb_counter = 0;
                            // _ = ch.next_form(None);
                        }
                    }
                    button.set_label("Continue");
                }
            }
            else {
                button.set_label("Submit");

                println!("counter: {}, reps: {}", ch.verb_counter, ch.reps_per_verb);

                let starting_vf = &ch.history[ch.history.len() - 2];
                let starting_form = starting_vf.get_form(false).unwrap().last().unwrap().form.to_string();

                let changed_vf = &ch.history[ch.history.len() - 1];

                change_label.set_text(format!("{:?} {:?} {:?} {:?} {:?}", changed_vf.person, changed_vf.number, changed_vf.tense, changed_vf.mood, changed_vf.voice).as_str());

                correct_label.set_markup("");
                starting_form_tv.buffer().set_text(&starting_form);
                changed_form_tv.buffer().set_text("");
                changed_form_tv.set_editable(true);
                changed_form_tv.set_cursor_visible(true);
                changed_form_tv.grab_focus();
            }
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hoplite Challenge")
        .default_width(550)
        .default_height(400)
        .content(&vbox)
        .build();

    //https://github.com/avranju/spacer-win/blob/65779df6e5a3884cc499cc2c48cd1348a7b4d63e/src/main.rs
    //https://stackoverflow.com/questions/72796392/how-to-bind-a-key-press-to-a-speccific-action-in-gtk4
    let evk = EventControllerKey::new();
    let tv = changed_form_tv_orig.clone();
    evk.connect_key_pressed(move |_evck, key, _code, _state| {
        
        let diacritic_option:Option<u32> = match key {
            Key::_1 => Some(HGK_ROUGH),
            Key::_2 => Some(HGK_SMOOTH),
            Key::_3 => Some(HGK_ACUTE),
            Key::_4 => Some(HGK_GRAVE),
            Key::_5 => Some(HGK_CIRCUMFLEX),
            Key::_6 => Some(HGK_MACRON),
            Key::_7 => None,
            Key::_8 => Some(HGK_IOTA_SUBSCRIPT),
            Key::_9 => None,
            Key::_0 => None,
            _ => None
        };

        if let Some(diacritic) = diacritic_option {
            let chars_to_get = 8;
            let buf = tv.buffer();
            let cursor_pos = buf.cursor_position();
            let mut iter_end = buf.iter_at_offset(cursor_pos);
            let start_pos = if cursor_pos >= chars_to_get { cursor_pos - chars_to_get } else { 0 };
            let mut iter_start = buf.iter_at_offset(start_pos);

            let s = buf.text(&iter_start, &iter_end, false);
            let new_s = hgk_toggle_diacritic_str_end(&s, diacritic, false, HgkUnicodeMode::PrecomposedPUA);

            buf.delete(&mut iter_start, &mut iter_end);
            buf.insert(&mut iter_start, &new_s);

            return Inhibit(true);
        }

        let translated_key = match key {
            Key::a | Key::A => "α",
            Key::b | Key::B => "β",
            Key::g | Key::G => "γ",
            Key::d | Key::D => "δ",
            Key::e | Key::E => "ε",
            Key::z | Key::Z => "ζ",
            Key::h | Key::H => "η",
            Key::u | Key::U => "θ",
            Key::i | Key::I => "ι",
            Key::k | Key::K => "κ",
            Key::l | Key::L => "λ",
            Key::m | Key::M => "μ",
            Key::n | Key::N => "ν",
            Key::o | Key::O => "ο",
            Key::j | Key::J => "ξ",
            Key::p | Key::P => "π",
            Key::r | Key::R => "ρ",
            Key::s | Key::S => "σ",
            Key::w | Key::W => "ς",
            Key::t | Key::T => "τ",
            Key::y | Key::Y => "υ",
            Key::f | Key::F => "φ",
            Key::x | Key::X => "χ",
            Key::c | Key::C => "ψ",
            Key::v | Key::V => "ω",
            _ => ""
        };

        if translated_key.len() > 0 {
            tv.emit_insert_at_cursor(translated_key);
            return Inhibit(true);
        }

        Inhibit(false)
    });
    changed_form_tv_orig.add_controller(&evk);

    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| {
        adw::init();
        load_css();
    });
    app.connect_activate(build_ui);
    app.run();
}
