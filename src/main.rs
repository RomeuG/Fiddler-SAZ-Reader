use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Label, ListStore, Orientation, TreeView, TreeViewColumn,
    WindowPosition,
};

use std::env;

// TODO: argument to define fixed width (and probably min width)
fn append_column(tree: &gtk::TreeView, title: &str, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    // column.set_sizing(gtk::TreeViewColumnSizing::);
    column.set_expand(true);

    column.set_title(&title);
    column.set_resizable(true);
    column.set_min_width(50);
    column.set_fixed_width(150);

    tree.append_column(&column);
}

fn build_ui(application: &gtk::Application, saz: Vec<sazparser::SazSession>) {
    let window = ApplicationWindow::new(application);
    window.set_resizable(false);

    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.set_property_height_request(300);
    scroll.set_property_width_request(300);

    let scroll2 = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll2.set_property_height_request(300);
    scroll2.set_property_width_request(300);

    let hbox = gtk::Box::new(Orientation::Horizontal, 0);

    // TODO: check how to do scrollable textviews
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

    scroll.add(&requestTextView);
    scroll2.add(&responseTextView);

    vbox.add(&scroll);
    vbox.add(&scroll2);

    // vbox.add(&requestTextView);
    // vbox.add(&responseTextView);

    let tree_view_model = gtk::TreeStore::new(&[u32::static_type(), String::static_type(), u32::static_type()]);

    let index_entries: Vec<u32> = saz.iter().map(|i| { i.index }).collect();
    let url_entries: Vec<String> = saz.iter().map(|i| { i.url.clone() }).collect();
    let body_entries: Vec<u32> = saz.iter().map(|i| { i.body }).collect();

    for i in 0..index_entries.len() {
        tree_view_model.insert_with_values(None, None, &[0, 1, 2], &[&index_entries[i], &url_entries[i], &body_entries[i]]);
    }

    let tree = gtk::TreeView::new();
    tree.set_property_margin(8);
    tree.set_property_width_request(300);
    tree.set_headers_visible(true);

    // Creating the two columns inside the view.
    append_column(&tree, "Index", 0);
    append_column(&tree, "Url", 1);
    append_column(&tree, "Body Size", 2);

    tree.set_model(Some(&tree_view_model));

    tree.connect_cursor_changed(move |tree_view| {
        let selection = tree_view.get_selection();
        println!("{:?}", selection);
        if let Some((model, iter)) = selection.get_selected() {

            let index = model.get_value(&iter, 0)
                .get::<u32>()
                .expect("jgirueahgiureahgrueia")
                .expect("graegjihreaoihgreioahgrae");

            println!("{}", index);
            let computed_index = (index - 1) as usize;

            requestTextView.get_buffer().unwrap().set_text(&saz[computed_index].file_request_contents);
            responseTextView.get_buffer().unwrap().set_text(&saz[computed_index].file_response_contents);
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
