// use std::io::Error;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::time::Duration;
// use std::thread;

// use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow};

// fn main() -> Result<(), Error> {
//     let term = Arc::new(AtomicBool::new(false));
//     signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
//     signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
//     while term.load(Ordering::Relaxed) {
//         println!("HI!");
//         thread::sleep(Duration::from_secs(2));

//         // let app = Application::builder().application_id(APP_ID).build();
//         // app.connect_activate(build_ui);
//         // app.run();
//     }

//     println!("HO!");
//     thread::sleep(Duration::from_secs(2));
//     Ok(())
// }

// fn build_ui(app: &Application) {
//     // Create a window and set the title
//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("My GTK App")
//         .build();

//     // Present window
//     window.present();
// }

use gtk::{prelude::*, Application, glib, ApplicationWindow};
use glib::{clone, MainContext};
use std::process;
use libc::{SIGINT, SIGTERM};

const APP_ID: &str = "be.allersma.Swappertje";

fn main() {
    // Create a new GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Swappertje")
            .default_width(400)
            .default_height(300)
            .build();
        window.show();
    });

    // Handle SIGINT and SIGTERM to gracefully quit the application
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

    // Run the application
    app.run();
}

fn build_window(app: &Application) {
    let window = ApplicationWindow::builder()
    .application(app)
    .title("Swappertje")
    .default_width(400)
    .default_height(300)
    .build();

    window.show();
}

// https://gtk-rs.org/gtk4-rs/git/book/hello_world.html