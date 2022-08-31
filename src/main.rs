use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::Button;
use gtk::StyleContext;
use gtk::CssProvider;
use gtk::gdk::Display;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn build_ui(app: &Application) {
    let text_view: gtk::TextView = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(false);

    let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .vexpand(true)
            .child(&text_view)
            .build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        vbox.set_homogeneous(false);
        //vbox.pack_start(&text_view, true, true, 0);
        //vbox.pack_start(&button, true, true, 0);
        vbox.append(&scrolled_window);
        vbox.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(550)
        .default_height(400)
        .child(&vbox)
        .build();

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
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
