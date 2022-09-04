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
    let text_view: gtk::TextView = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(true);
    text_view.set_justification(gtk::Justification::Center);
    let margin = 10;
    text_view.set_left_margin(margin);
    text_view.set_top_margin(margin);
    text_view.set_right_margin(margin);
    text_view.set_bottom_margin(margin);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .vexpand(true)
        .child(&text_view)
        .build();

    let text_view2: gtk::TextView = gtk::TextView::new();
    text_view2.set_wrap_mode(gtk::WrapMode::Word);
    text_view2.set_cursor_visible(false);
    text_view2.set_justification(gtk::Justification::Center);
    text_view2.set_left_margin(margin);
    text_view2.set_top_margin(margin);
    text_view2.set_right_margin(margin);
    text_view2.set_bottom_margin(margin);
    text_view2.set_editable(false);

    //text_view2.buffer().set_text("\nφέρω");
        
    let scrolled_window2 = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .vexpand(true)
        .child(&text_view2)
        .build();

    let label = gtk::Label::new(Some(""));
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
    vbox.append(&scrolled_window2);
    vbox.append(&label);
    vbox.append(&scrolled_window);
    vbox.append(&correct_label);
    vbox.append(&button);

    let chooser = Arc::new(Mutex::new(init_random_form_chooser("../hoplite_verbs_rs/testdata/pp.txt", 20)));
    if let Ok(mut ch) = chooser.lock() {
        ch.set_reps_per_verb(4);
    }
    let tv2 = text_view2.clone();
    let tv1 = text_view.clone();
    button.connect_clicked(move |button| {
        if let Ok(mut ch) = chooser.lock() {
            if ch.history.len() == 0 {
                _ = ch.next_form(None);
            }

            if button.label().unwrap() == "Submit" {
                let answer = tv1.buffer().text(&tv1.buffer().start_iter(), &tv1.buffer().end_iter(), false);
                let prev1 = &ch.history[ch.history.len() - 1]; //call here before calling next_form()
                let form = prev1.get_form(false).unwrap().last().unwrap().form.to_string();
                if let Ok(vf) = ch.next_form(Some(&answer)) {
                    let is_correct = vf.1;
                    if let Some(ic) = is_correct {
                        if ic {
                            println!("correct");
                            correct_label.set_markup("<span foreground=\"green\">correct</span>");
                        }
                        else {
                            println!("incorrect");
                            
                            correct_label.set_markup(format!("<span foreground=\"red\">incorrect: {}</span>", form).as_str());
                        }
                    }
                    button.set_label("Continue");
                }
            }
            else {
                button.set_label("Submit");

                println!("counter: {}, reps: {}", ch.verb_counter, ch.reps_per_verb);
                // if ch.verb_counter == 1 {
                //     _ = ch.next_form(None); //change to...
                //     //ch.verb_counter = 6;
                //     println!("change");
                // }

                let prev1 = &ch.history[ch.history.len() - 2];
                let form = prev1.get_form(false).unwrap().last().unwrap().form.to_string();

                let prev = &ch.history[ch.history.len() - 1];
                //let prev_f = prev.get_form(false).unwrap().last().unwrap().form.to_string();
                //println!("prev {}", prev_f);

                label.set_text(format!("{:?} {:?} {:?} {:?} {:?}", prev.person, prev.number, prev.tense, prev.mood, prev.voice).as_str());

                correct_label.set_markup("");
                tv2.buffer().set_text(&form);
                tv1.buffer().set_text("");
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
    let tv = text_view.clone();
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
    text_view.add_controller(&evk);

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
