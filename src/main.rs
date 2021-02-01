use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Label, ListStore, Orientation, TreeView, TreeViewColumn,
    WindowPosition,
};

use std::env;

fn append_column(tree: &gtk::TreeView, title: &str, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    column.set_title(&title);

    tree.append_column(&column);
}

fn build_ui(application: &gtk::Application, saz: Vec<sazparser::SazSession>) {
    let window = ApplicationWindow::new(application);
    window.set_resizable(false);

    let hbox = gtk::Box::new(Orientation::Horizontal, 0);

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

    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.set_margin_start(8);
    vbox.set_margin_end(8);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);

    vbox.add(&requestTextView);
    vbox.add(&responseTextView);

    let tree_view_model = gtk::ListStore::new(&[String::static_type(), String::static_type()]);

    let index_entries: Vec<u32> = saz.iter().map(|i| { i.index }).collect();
    let url_entries: Vec<String> = saz.iter().map(|i| { i.url.clone() }).collect();
    let body_entries: Vec<u32> = saz.iter().map(|i| { i.body }).collect();

    // let request_entries: Vec<std::rc::Rc<String>> = saz.iter().map(|a| { a.file_request_contents.clone() }).collect();
    // let response_entries: Vec<std::rc::Rc<String>> = saz.iter().map(|a| { a.file_response_contents.clone() }).collect();

    let test_entries1 = &["Test1", "Test2", "Test3"];
    let test_entries2 = &["Test1", "Test2", "Test3"];

    for i in 0..3 {
        tree_view_model.insert_with_values(None, &[0, 1], &[&test_entries1[i], &test_entries2[i]]);
    }

    let tree = gtk::TreeView::new();
    tree.set_property_margin(8);
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

            let column_zero = model.get_value(&iter, 0)
                .get::<String>()
                .expect("Treeview selection, column 0")
                .expect("Treeview selection, column 0: mandatory value not found");

            let column_first = model.get_value(&iter, 1)
                .get::<String>()
                .expect("Treeview selection, column 1")
                .expect("Treeview selection, column 1: mandatory value not found");

            println!("Hello '{}' from row {}", &column_first, &column_zero);

            requestTextView.get_buffer().unwrap().set_text(&column_zero);
            responseTextView.get_buffer().unwrap().set_text(&column_first);
        }
    });

    hbox.add(&tree);
    hbox.add(&vbox);

    window.add(&hbox);

    window.show_all();
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        // TODO change this later
        println!("File argument is missing.");
        std::process::exit(1);
    }

    println!("Parsing file...");
    let saz = sazparser::parse(&*args[1]);

    match saz {
        Ok(v) => {
            let application = gtk::Application::new(
                Some("com.romeug.fiddlerreader"),
                Default::default(),
            ).expect("Initialization failed...");

            application.connect_activate(move |app| {
                build_ui(app, v.clone());
            });

            application.run(&vec![]);
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
