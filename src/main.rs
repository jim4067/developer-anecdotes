#[macro_use]
extern crate cascade;

use async_channel::Sender;
use gtk::prelude::*;
use rand::{thread_rng, Rng};
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

    let app = App::new(tx);

    let event_handler = async move {
        while let Ok(click_event) = rx.recv().await {
            match click_event {
                Event::Clicked => {
                    let new_anecdote = random_anecdote();
                    app.anecdote.set_label(new_anecdote.into());
                }
            }
        }
    };

    glib::MainContext::default().spawn_local(event_handler);
    gtk::main();
}

// pub fn spawn<F>(future: F)
// where
//     F: Future<Output = ()> + 'static,
// {
//     glib::MainContext::default().spawn_local(future)
// }

pub fn spawn<F: Future<Output = ()> + 'static>(future: F) {
    glib::MainContext::default().spawn_local(future)
}

struct App {
    pub anecdote: gtk::Label,
}

impl App {
    pub fn new(tx: Sender<Event>) -> Self {
        //text widget to display the anecdote down under
        let anecdote = cascade! {
            gtk::Label::new(random_anecdote().into());
            ..set_halign(gtk::Align::Center);
        };

        let button = cascade! {
            gtk::Button::with_label("new anecdote");
            ..get_style_context().add_class("destructive-action");
            ..set_border_width(4);
            ..set_halign(gtk::Align::Center);
            ..set_hexpand(false);
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
            ..pack_start(&anecdote, true, true, 10);
            ..pack_start(&button, false, false, 10);
            ..show_all();
        };

        //a touch of gtk magic to make the window appear
        let _window = cascade! {
            gtk::Window::new(gtk::WindowType::Toplevel);
            ..add(&container);
            ..set_title("developer anecdotes");
            ..set_default_size(300, 250);
            ..set_position(gtk::WindowPosition::CenterAlways);
            ..connect_delete_event(move |_,_|{
                gtk::main_quit();
                gtk::Inhibit(false)
            });
            ..show_all();
        };

        gtk::Window::set_default_icon_name("jaw dropping icon name");

        Self { anecdote }
    }
}

fn random_anecdote(/*list: &[&str]*/) -> &'static str {
    const ANECDOTES_LIST : [&str; 6] = [
        "If it hurts, do it more often",
        "Adding manpower to a late software project makes it later!",
        "The first 90 percent of the code accounts for the first 90 percent of the development time...The remaining 10 percent of the code accounts for the other 90 percent of the development time.",
        "Any fool can write code that a computer can understand. Good programmers write code that humans can understand.",
        "Premature optimization is the root of all evil.",
        "Debugging is twice as hard as writing the code in the first place. Therefore, if you write the code as cleverly as possible, you are, by definition, not smart enough to debug it."
    ];

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..6);
    ANECDOTES_LIST[random_index]
}

//this is the implementation after learning about the event driven thingy
