use gtk::{prelude::*, Application, glib, ApplicationWindow};
use gtk::{DropDown, Button, Entry, Box, Orientation};
use gtk::{ScrolledWindow, PasswordEntry, Label};
use glib::{clone, MainContext};
use libc::{SIGINT, SIGTERM};
use nix::unistd::Uid;
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Mutex, Arc};
use std::io::Write;
use std::thread;
use std::time::Duration;

const APP_ID: &str = "be.allersma.Swappertje";

fn main() {
    if !Uid::effective().is_root() {
        let args: Vec<_> = env::args().collect();

        let app = Application::builder()
        .application_id(APP_ID)
        .build();
        
        match ask_password(&app) {
            Some(password) => {
                let mut after_auth_proc = Command::new("sudo")
                    .args(["-S", &args[0]])
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Failed to run with elevated privileges.");

                if let Some(stdin) = after_auth_proc.stdin.as_mut() {
                    stdin
                        .write_all(format!("{}\n", password).as_bytes())
                        .expect("Failed to write to stdin.");
                }

                after_auth_proc.wait().expect("Failed to wait for process.");
            },
            _ => println!("No password entered.")
        }
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

fn ask_password(app: &Application) -> Option<String> {
    let password: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let app_clone = app.clone();
    let passwd_cln = password.clone();

    app.connect_activate(move |app_clone: &Application| {
        let mut msg = String::from("Swappertje requires elevated privileges to work.\n");
        msg.push_str("Enter the sudo password to use Swappertje with elevated privileges.");
        let label = Label::new(Some(&msg));
        let entry = PasswordEntry::new();
    
        let vbox = Box::new(Orientation::Horizontal, 10);
    
        let cancel = Button::with_label("Cancel");
        let app_cancel_clone = app_clone.clone();
        cancel.connect_clicked(move |_| {
            app_cancel_clone.quit();
        });
        vbox.append(&cancel);
        
        let confirm = Button::with_label("Confirm");
        let password_clone = Arc::clone(&passwd_cln);
        let app_confirm_clone = app_clone.clone();
        let entry_clone = entry.clone();
        confirm.connect_clicked(move |_| {
            let mut passwd = password_clone.lock().unwrap();
            *passwd = Some(String::from(entry_clone.text().to_string()));
            app_confirm_clone.quit();
        });
        vbox.append(&confirm);
    
        let hbox = Box::new(Orientation::Vertical, 10);
        hbox.append(&label);
        hbox.append(&entry);
        hbox.append(&vbox);
    
        let window = ApplicationWindow::builder()
        .application(app_clone)
        .title("Swappertje")
        .child(&hbox)
        .build();
    
        window.present();
    });
    app.run();

    let passwd = password.lock().unwrap();

    passwd.clone()
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