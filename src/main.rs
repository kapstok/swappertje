use gtk::{prelude::*, Application, glib, ApplicationWindow};
use gtk::{DropDown, Button, Entry, Box, Orientation};
use gtk::{ScrolledWindow};
use glib::{clone, MainContext};
use libc::{SIGINT, SIGTERM};

mod elevated;
mod swap;

const APP_ID: &str = "be.allersma.Swappertje";

fn main() {
    if !elevated::check_for_elevated() {
        return;
    }

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_window);
    app.connect_shutdown(on_quit_app);
    handle_signals(&app);
    app.run();
}

fn build_window(app: &Application) {
    let layout = build_window_layout();

    let window = ApplicationWindow::builder()
    .application(app)
    .title("Swappertje")
    .default_width(400)
    .default_height(300)
    .child(&layout)
    .build();

    window.present();
}

fn build_window_layout() -> Box {
    let new_swap_box = build_new_swap_box();
    let scrolled_win = build_scrolled_window();

    let result = Box::new(Orientation::Vertical, 10);
    result.append(&new_swap_box);
    result.append(&scrolled_win);

    result
}

fn build_scrolled_window() -> ScrolledWindow {
    let result = ScrolledWindow::new();
    result.set_vexpand(true);
    let vbox = Box::new(Orientation::Vertical, 10);

    for i in 0..20 {
        let btn = Button::with_label(&format!("Button {}", i + 1));
        vbox.append(&btn);
    }

    result.set_child(Some(&vbox));
    result
}

fn build_new_swap_box() -> Box {
    let entry = Entry::builder()
    .build();

    let optn = ["MB", "GB"];
    let dropdown = DropDown::from_strings(&optn);

    let btn = Button::with_label(&"Add swap memory");
    btn.connect_clicked(|_button| {

    });

    let result = Box::new(Orientation::Horizontal, 10);
    result.append(&entry);
    result.append(&dropdown);
    result.append(&btn);

    result
}

fn on_quit_app(_app: &Application) {
    println!("Application is quitting...");
}

fn handle_signals(app: &Application) {
    let app_clone = app.clone();
    MainContext::default().spawn_local(async move {
        glib::unix_signal_add_local(SIGINT, clone!(@strong app_clone => move || {
            println!("Received SIGINT, quitting...");
            app_clone.quit();
            glib::ControlFlow::Break
        }));

        glib::unix_signal_add_local(SIGTERM, clone!(@strong app_clone => move || {
            println!("Received SIGTERM, quitting...");
            app_clone.quit();
            glib::ControlFlow::Break
        }));
    });
}