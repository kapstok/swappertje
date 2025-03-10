use gtk::{prelude::*, Application, Button, ApplicationWindow};
use gtk::{Box, PasswordEntry, Orientation, Label};
use nix::unistd::Uid;
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Mutex, Arc};
use std::io::Write;

const APP_ID: &str = "be.allersma.SwappertjeAuth";

pub fn check_for_elevated() -> bool {
    let is_root: bool = Uid::effective().is_root();
    if !is_root {
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
            },
            _ => println!("No password entered.")
        }
    }

    is_root
}

#[allow(unused_variables)]
fn ask_password(app: &Application) -> Option<String> {
    let password: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let app_clone = app.clone();
    let passwd_cln = password.clone();

    app.connect_activate(move |app_clone: &Application| {
        let mut msg = String::from("Swappertje requires elevated privileges to work.\n");
        msg.push_str("Enter the sudo password to use Swappertje with elevated privileges.\n");
        msg.push_str("Try to run this program as root next time.");
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