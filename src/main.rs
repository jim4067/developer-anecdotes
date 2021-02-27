extern crate gtk;
extern crate rand;

use gtk::*;
use rand::{thread_rng, Rng};
use std::process;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

const ANECDOTES_LIST : [&str; 6] = [
    "If it hurts, do it more often",
    "Adding manpower to a late software project makes it later!",
    "The first 90 percent of the code accounts for the first 90 percent of the development time...The remaining 10 percent of the code accounts for the other 90 percent of the development time.",
    "Any fool can write code that a computer can understand. Good programmers write code that humans can understand.",
    "Premature optimization is the root of all evil.",
    "Debugging is twice as hard as writing the code in the first place. Therefore, if you write the code as cleverly as possible, you are, by definition, not smart enough to debug it."
];

fn main() {
    if gtk::init().is_err() {
        eprintln!("failed to init the GTK app");
        process::exit(1);
    }
    //setting the atomically referenced var
    let random = gen_random();

    let index = Arc::new(AnecdoteComponent::new(random));

    let app = App::new(&index);

    {
        let index_label = app.content.index_label.clone();
        let anecdote = app.content.anecdote.clone();
        app.content.next.clone().connect_clicked(move |_| {
            let new_index = gen_random();
            anecdote.set_label(ANECDOTES_LIST[new_index]);
            index_label.set_label(format!("{} / 6", new_index.to_string().as_str()).as_str());
        });
    }

    app.window.show_all();
    gtk::main();
}

struct App {
    pub header: Header,
    pub window: Window,
    pub content: Content,
}

struct Header {
    pub header_container: HeaderBar,
}

struct Content {
    pub anecdote: Label,
    pub container: Box,
    index_label: Label,
    pub next: Button,
}

struct AnecdoteComponent(AtomicUsize);

impl App {
    fn new(index: &AnecdoteComponent) -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new(index);

        window.set_title("developer anecdotes");
        window.set_default_size(550, 250);
        window.set_position(WindowPosition::CenterAlways);
        Window::set_default_icon_name("developer anecdotes");

        //add the content box to the window
        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }
}

impl Header {
    fn new() -> Header {
        let header_container = HeaderBar::new();

        // container.set_title(Some("developer anecdotes"));
        header_container.set_show_close_button(true);

        Header { header_container }
    }
}

impl Content {
    fn new(index: &AnecdoteComponent) -> Content {
        let container = Box::new(Orientation::Vertical, 0);

        let anecdote_container = Box::new(Orientation::Horizontal, 0);
        let index_str = index.get_index();
        let index_label = format!("{} / 6 ->", index_str).to_string(); //concatenating in rust is not a walk in the park/ do not also get fond of the compiler making assumptions of your types
        let index_label = Label::new(Some(&index_label));
        let index = index.get_index();
        let anecdote = Label::new(Some(ANECDOTES_LIST[index]));

        index_label.set_halign(Align::Start);
        anecdote.set_halign(Align::Center);
        anecdote_container.pack_start(&index_label, false, false, 5);
        anecdote_container.pack_start(&anecdote, true, true, 5);

        // let button_container = Box::new(Orientation::Horizontal, 0);
        let next = Button::with_label("next");
        next.get_style_context().add_class("destructive-action");
        anecdote_container.pack_end(&next, true, true, 5);

        container.pack_start(&anecdote_container, true, false, 0);

        Content {
            anecdote,
            container,
            index_label,
            next,
        }
    }
}

impl AnecdoteComponent {
    fn new(index: usize) -> AnecdoteComponent {
        AnecdoteComponent(AtomicUsize::new(index))
    }
    fn get_index(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

//the function for generating the random numbers
fn gen_random() -> usize {
    let mut rng = thread_rng();
    let rand_index = rng.gen_range(0..6);
    rand_index
}
