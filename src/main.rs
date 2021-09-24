#[macro_use]
extern crate cascade;

use async_channel::Sender;
use gtk::prelude::*;
use std::future::Future;
use std::process;

enum Event {
    Clicked,
}

fn main() {
    glib::set_program_name("developer anecdotes".into());
    glib::set_application_name("developer anecdotes");

    if gtk::init().is_err() {
        eprintln!("Error ! Failed to start gtk application");
        process::exit(1);
    }

    let (tx, rx) = async_channel::unbounded();

    let event_handler = async move {
        while let Ok(message) = rx.recv().await {
            match message {
                Event::Clicked => {}
            }
        }
    };
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    glib::MainContext::default().spawn_local(future)
}

struct App {
    pub anecdote: gtk::Label,
    pub button: gtk::Button,
    // pub window: gtk::Window,
}

impl App {
    pub fn new(tx: Sender<Event>, anecdote: &str) -> Self {
        //text widget to display the anecdote down under
        let anecdote = gtk::Label::new(anecdote.into());

        let button = cascade! {
            gtk::Button::with_label("next");
            ..get_style_context().add_class("destructive-action");
            ..set_border_width(4);
            ..connect_clicked(move |_|{
                let tx = tx.clone();
                spawn(async move {
                    let _ = tx.send(Event::Clicked).await;
                });
            });
        };

        //will stitch up everything
        let container = cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 0);
            ..add(&anecdote);
            ..add(&button);
            ..show_all();
        };

        //a touch of gtk magic to make the window appear
        let _window = cascade! {
            gtk::Window::new(gtk::WindowType::Toplevel);
            ..add(&container);
            ..set_title("developer anecdotes");
            ..set_default_size(400, 250);
            ..set_position(gtk::WindowPosition::CenterAlways);
            ..connect_delete_event(move |_,_|{
                gtk::main_quit();
                gtk::Inhibit(false)
            });
            ..show_all();
        };

        gtk::Window::set_default_icon_name("jaw dropping icon name");

        Self { anecdote, button }
    }
}

// struct Content {
//     anecdote: gtk::
// }

//instead create a function to randomly select an anecdote to spit out once selected
