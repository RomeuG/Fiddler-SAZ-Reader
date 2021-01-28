use relm_derive::Msg;
use relm_derive::widget;
use relm::{connect, Relm, Update, Widget};
use gtk::Orientation;
use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};

#[derive(Msg)]
enum Msg {
    // …
    Quit,
}

struct Model {
    // relm: relm::Relm<Win>
}

struct Win {
    // …
    model: Model,
    window: Window,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

fn append_column(tree: &gtk::TreeView, title: &str, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    column.set_title(&title);

    tree.append_column(&column);
}

impl Widget for Win {

    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);
        window.set_resizable(false);

        let hbox = gtk::Box::new(Orientation::Horizontal, 0);

	// let table = gtk::TextView::new();
	// table.set_property_width_request(300);
	// table.set_margin_start(8);
	// table.set_margin_top(8);
	// table.set_margin_bottom(8);
	// table.get_buffer().unwrap().set_text("Table text!");

	let tree_view_model = gtk::ListStore::new(&[String::static_type(), String::static_type()]);

	let test_entries1 = &["Test1", "Test2", "Test3"];
	let test_entries2 = &["Test1", "Test2", "Test3"];

	for i in 0..3 {
	    tree_view_model.insert_with_values(None, &[0, 1], &[&test_entries1[i], &test_entries2[i]]);
	}

	let tree = gtk::TreeView::new();
	tree.set_property_width_request(300);
	tree.set_headers_visible(true);
	// Creating the two columns inside the view.
	append_column(&tree, "Title1", 0);
	append_column(&tree, "Title2", 1);

	tree.set_model(Some(&tree_view_model));

        tree.connect_cursor_changed(move |tree_view| {
            let selection = tree_view.get_selection();
            println!("{:?}", selection);
            if let Some((model, iter)) = selection.get_selected() {
                println!("Hello '{}' from row {}",
                         model
                         .get_value(&iter, 1)
                         .get::<String>()
                         .expect("Treeview selection, column 1")
                         .expect("Treeview selection, column 1: mandatory value not found"),
                         model
                         .get_value(&iter, 0)
                         .get::<String>()
                         .expect("Treeview selection, column 0")
                         .expect("Treeview selection, column 0: mandatory value not found"),
                );
            }
        });

        let vbox = gtk::Box::new(Orientation::Vertical, 0);
	vbox.set_margin_start(8);
	vbox.set_margin_end(8);
	vbox.set_margin_top(8);
	vbox.set_margin_bottom(8);

	let requestTextView = gtk::TextView::new();
	requestTextView.set_property_height_request(300);
	requestTextView.set_property_width_request(300);
	requestTextView.set_editable(false);
	requestTextView.get_buffer().unwrap().set_text("Request text!");

	let responseTextView = gtk::TextView::new();
	responseTextView.set_property_height_request(300);
	responseTextView.set_property_width_request(300);
	responseTextView.set_editable(false);
	responseTextView.set_margin_top(8);
	responseTextView.get_buffer().unwrap().set_text("Response text!");

	vbox.add(&requestTextView);
	vbox.add(&responseTextView);

	hbox.add(&tree);
	hbox.add(&vbox);

	window.add(&hbox);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            window,
        }
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let sazfile = &args[1];

    // do saz parsing here before going to the GUI

    Win::run(()).unwrap();
}
