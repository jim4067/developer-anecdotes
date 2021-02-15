extern crate gtk;

use gtk::*;
use std::process;

fn main() {
    if gtk::init().is_err() {
        process::exit(1);
    }
    //else init gtk application
    let app = App::new();
    app.window.show_all();
    gtk::main();
}

struct App {
    pub header: Header,
    pub window: Window,
}

struct Header {
    pub header_container: HeaderBar,
}

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        window.set_title("developer anecdotes");
        window.set_default_size(650, 150);
        Window::set_default_icon_name("developer anecdotes");

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header }
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
