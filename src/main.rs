use gtk::prelude::*;
use gtk::{Application};
use gtk::Button;
use gtk::StyleContext;
use gtk::CssProvider;
use gtk::gdk::Display;
use adw::{ApplicationWindow, HeaderBar};
//use gtk::Inhibit;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

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

    // text_view.connect("key-press", false, |values| {
    //     println!("key pressed");
    //     //Inhibit(false)
    //     None
    // });
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

    text_view2.buffer().set_text("\nφέρω");
        
    let scrolled_window2 = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .vexpand(true)
        .child(&text_view2)
        .build();

    let label = gtk::Label::new(Some(""));
    label.set_markup("First <b>Singular</b> Present <b>Active</b> Indicative");

    // Create a button with label and margins
    let button = Button::builder()
        .label("Enter")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_button| {
        // Set the label to "Hello World!" after the button has been clicked on
        //button.set_label("Hello World!");
    });

    let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        vbox.set_homogeneous(false);
        //vbox.pack_start(&text_view, true, true, 0);
        //vbox.pack_start(&button, true, true, 0);
        vbox.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("Hoplite Challenge", ""))
                .build(),
        );
        vbox.append(&scrolled_window2);
        vbox.append(&label);
        vbox.append(&scrolled_window);
        vbox.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hoplite Challenge")
        .default_width(550)
        .default_height(400)
        .content(&vbox)
        .build();

    //     window
    // .connect("key_press_event", false, |values| {
    //     // "values" is a 2-long slice of glib::value::Value, which wrap G-types
    //     // You can unwrap them if you know what type they are going to be ahead of time
    //     // values[0] is the window and values[1] is the event
    //     let raw_event = &values[1].get::<gtk::gdk::Event>().unwrap().unwrap();
    //     // You have to cast to the correct event type to access some of the fields
    //     match raw_event.downcast_ref::<gtk::gdk::EventKey>() {
    //         Some(event) => {
    //             println!("key value: {:?}", std::char::from_u32(event.get_keyval()));
    //             println!("modifiers: {:?}", event.get_state());
    //         },
    //         None => {},
    //     }

    //     // You need to return Some() of a glib Value wrapping a bool
    //     let result = gtk::glib::value::Value::from_type(gtk::glib::types::Type::Bool);
    //     // I can't figure out how to actually set the value of result
    //     // Luckally returning false is good enough for now.
    //     Some(result)
    // })
    // .unwrap();

    // Present window
    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| {
        adw::init();
        load_css();
    });
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
