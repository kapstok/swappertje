use gtk::{prelude::*, Application, glib, ApplicationWindow};
use gtk::{DropDown, Button, Entry, Box, Orientation};
use gtk::{ScrolledWindow};
use glib::{clone, MainContext};
use libc::{SIGINT, SIGTERM};

mod elevated;
mod swap;

static mut SWAPFILES: u64 = 0;

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
    // let scrolled_win = build_scrolled_window();

    let result = Box::new(Orientation::Vertical, 10);
    result.append(&new_swap_box);
    // result.append(&scrolled_win);

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

#[allow(static_mut_refs)]
fn build_new_swap_box() -> Box {
    let entry = Entry::builder()
    .build();

    let optn = ["MB", "GB"];
    let dropdown = DropDown::from_strings(&optn);

    let btn = Button::with_label(&"Add swap memory");
    let entry_clone = entry.clone();
    let dropdown_clone = dropdown.clone();
    btn.connect_clicked(move |_button| {
        let filesize = entry_clone.text().parse::<i64>();

        match filesize {
            Ok(size) => {
                unsafe {
                let mut filename: String = String::from("/opt/swappertje/swp/swappertje_");
                filename.push_str(&SWAPFILES.to_string());
                let sz: i64 = if dropdown_clone.selected() == 0 {
                    size * 1024 * 1024
                } else {
                    size * 1024 * 1024 * 1024
                };
                let swapfile = swap::Swapfile::create(&filename, &sz);
                match swapfile {
                    Ok(_file) => SWAPFILES += 1,
                    Err(_) => {}
                }
                }
            },
            Err(_) => {}
        }
    });

    let result = Box::new(Orientation::Horizontal, 10);
    result.append(&entry);
    result.append(&dropdown);
    result.append(&btn);

    result
}

#[allow(static_mut_refs)]
fn on_quit_app(_app: &Application) {
    unsafe {
        for i in 0..SWAPFILES {
            println!("Destroying swapfile {}/{}", i + 1, &SWAPFILES);
            let mut filename = String::from("/opt/swappertje/swp/swappertje_");
            filename.push_str(&i.to_string());

            match swap::destroy(&filename) {
                Ok(_) => {},
                Err(e) => println!("Failed to destroy {}\nDetails: {:?}", filename, e)
            }
        }
    }
    println!("Done!");
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