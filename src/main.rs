use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Orientation};

use std::env;

fn replace<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();

    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            i += to_len;
        } else {
            i += 1;
        }
    }

    result
}

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
    window.set_resizable(true);

    let scroller_tree = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroller_tree.set_property_width_request(300);
    scroller_tree.set_min_content_width(300);
    scroller_tree.set_min_content_height(300);
    scroller_tree.set_hexpand(true);
    scroller_tree.set_vexpand(true);

    let scroller_request = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroller_request.set_property_height_request(300);
    scroller_request.set_property_width_request(300);
    scroller_request.set_hexpand(true);
    scroller_request.set_vexpand(true);

    let scroller_response = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroller_response.set_property_height_request(300);
    scroller_response.set_property_width_request(300);
    scroller_response.set_hexpand(true);
    scroller_response.set_vexpand(true);

    let hbox = gtk::Box::new(Orientation::Horizontal, 0);

    let request_text_view = gtk::TextView::new();
    request_text_view.set_property_height_request(300);
    request_text_view.set_property_width_request(300);
    request_text_view.set_editable(false);
    request_text_view
        .get_buffer()
        .unwrap()
        .set_text("Request text!");

    let response_text_view = gtk::TextView::new();
    response_text_view.set_property_height_request(300);
    response_text_view.set_property_width_request(300);
    response_text_view.set_editable(false);
    response_text_view.set_margin_top(8);
    response_text_view
        .get_buffer()
        .unwrap()
        .set_text("Response text!");

    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.set_margin_start(8);
    vbox.set_margin_end(8);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);

    scroller_request.add(&request_text_view);
    scroller_response.add(&response_text_view);

    vbox.add(&scroller_request);
    vbox.add(&scroller_response);

    let tree_view_model = gtk::TreeStore::new(&[
        u32::static_type(),
        String::static_type(),
        u32::static_type(),
    ]);

    let index_entries: Vec<u32> = saz.iter().map(|i| i.index).collect();
    let url_entries: Vec<String> = saz.iter().map(|i| i.url.clone()).collect();
    let body_entries: Vec<u32> = saz.iter().map(|i| i.body).collect();

    for i in 0..index_entries.len() {
        tree_view_model.insert_with_values(
            None,
            None,
            &[0, 1, 2],
            &[&index_entries[i], &url_entries[i], &body_entries[i]],
        );
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

        if let Some((model, iter)) = selection.get_selected() {
            let index = model
                .get_value(&iter, 0)
                .get::<u32>()
                .expect("Result unwrap failed.")
                .expect("Option unwrap failed.");

            let computed_index = (index - 1) as usize;

            // TODO: try to understand what byte should replace null bytes
            let req_bytes = &saz[computed_index].file_request_contents.as_bytes();
            let req_bytes_replaced = replace(&req_bytes[..], &[0], &[1]);
            let req_string = std::string::String::from_utf8_lossy(&req_bytes_replaced);

            // TODO: try to understand what byte should replace null bytes
            let res_bytes = &saz[computed_index].file_response_contents.as_bytes();
            let res_bytes_replaced = replace(&res_bytes[..], &[0], &[1]);
            let res_string = std::string::String::from_utf8_lossy(&res_bytes_replaced);

            request_text_view
                .get_buffer()
                .unwrap()
                .set_text(&req_string);
            response_text_view
                .get_buffer()
                .unwrap()
                .set_text(&res_string);
        }
    });

    scroller_tree.add(&tree);
    hbox.add(&scroller_tree);
    hbox.add(&vbox);

    window.add(&hbox);

    window.show_all();
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("File argument is missing.");
        std::process::exit(1);
    }

    println!("Parsing file...");
    let saz = sazparser::parse(&*args[1]);

    match saz {
        Ok(v) => {
            let application =
                gtk::Application::new(Some("com.romeug.fiddlerreader"), Default::default())
                    .expect("Initialization failed...");

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
