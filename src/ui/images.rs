use cursive::event::Key;
use cursive::traits::*;
use cursive_table_view::{TableView, TableViewItem};

use cursive::view::Margins;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

use log::info;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::sync::Arc;

use crate::app::App;
use crate::func::images;
use crate::utils;

type ImagesTable = TableView<ImageTableRow, ImageTableColumn>;

#[derive(Debug)]
enum ImageTableRowAction {
    Delete,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageTableColumn {
    ID,
    Name,
    Size,
}

impl ImageTableColumn {
    #[allow(dead_code)]
    fn as_str(&self) -> &str {
        match *self {
            ImageTableColumn::ID => "ID",
            ImageTableColumn::Name => "Name",
            ImageTableColumn::Size => "Size",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageTableRow {
    id: String,
    name: String,
    size: u64,
}

impl TableViewItem<ImageTableColumn> for ImageTableRow {
    fn to_column(&self, column: ImageTableColumn) -> String {
        match column {
            ImageTableColumn::ID => self.id.to_string(),
            ImageTableColumn::Name => self.name.to_string(),
            ImageTableColumn::Size => utils::humanize_size(self.size, None),
        }
    }

    fn cmp(&self, other: &Self, column: ImageTableColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            ImageTableColumn::ID => self.id.cmp(&other.id),
            ImageTableColumn::Name => self.name.cmp(&other.name),
            ImageTableColumn::Size => self.size.cmp(&other.size),
        }
    }
}

pub fn render_images_table(ui: &mut Cursive) {
    // Determine table.
    let mut table = ImagesTable::new()
        .column(ImageTableColumn::ID, "ID", |c| c.width_percent(40))
        .column(ImageTableColumn::Name, "Name", |c| c.width_percent(40))
        .column(ImageTableColumn::Size, "Size", |c| c.width_percent(20));
    // Set callbacks.
    add_table_row_callback(&mut table);

    // Add table to the layout.
    ui.add_fullscreen_layer(table.with_name("images_table").full_width().full_height());

    // Fetch images.
    fetch_table_data(ui, "images_table", None);

    ui.clear_global_callbacks('m');
    ui.add_global_callback('m', |ui: &mut Cursive| show_table_row_menu(ui));
}

fn fetch_table_data(ui: &mut Cursive, table: &str, selected_row: Option<usize>) {
    let mut rows = vec![];
    let app = ui.user_data::<Arc<RefCell<App>>>().unwrap();
    let images_fut = images::get_images(app.borrow().docker.clone());
    let images = app
        .borrow_mut()
        .runtime
        .block_on(images_fut)
        .expect("Couldn't fetch Docker images.");

    // Fill up rows from fetched images.
    for image in images {
        rows.push(ImageTableRow {
            id: image.id,
            name: image.repo_tags.map_or("".to_string(), |i| i.join(",")),
            size: image.size,
        })
    }

    // Compile table.
    ui.call_on_name(table, |table: &mut ImagesTable| {
        table.set_items(rows);
        // Preselect row.
        if let Some(mut row) = selected_row {
            // Case when the selected row was the last one in table -> we select one row above.
            if table.len() == row + 1 {
                row = row - 1;
            }
            table.set_selected_row(row);
        }
    });
}

/// Sets image table row callback - the menu.
fn add_table_row_callback(table: &mut ImagesTable) {
    let show_table_row_menu_cb = |ui: &mut Cursive| show_table_row_menu(ui);

    table.set_on_select(move |ui: &mut Cursive, _row: usize, _index: usize| {
        ui.clear_global_callbacks('m');
        ui.add_global_callback('m', show_table_row_menu_cb);
    });
}

/// Shows image table item menu. Also sets callback for it's hidding.
fn show_table_row_menu(ui: &mut Cursive) {
    let table_row_menu_item_callback_cb =
        |ui: &mut Cursive, value: &ImageTableRowAction| table_row_menu_item_callback(ui, value);

    let mut menu = SelectView::new();
    menu.add_item("Delete", ImageTableRowAction::Delete);
    menu.set_on_submit(table_row_menu_item_callback_cb);
    ui.add_layer(
        Dialog::around(menu)
            .title("Menu")
            .padding(Margins::lrtb(4, 4, 1, 1))
            .with_name("image_table_row_menu"),
    );
    ui.add_global_callback(Key::Esc, |ui| {
        ui.pop_layer();
        ui.clear_global_callbacks(Key::Esc);
    });
}

/// Catches image table item menu select view choice - see `ImageTableRowAction`.
fn table_row_menu_item_callback(ui: &mut Cursive, value: &ImageTableRowAction) {
    info!("VALUE: {:?}", value);

    // Hide the menu dialog.
    ui.pop_layer();
    let app = ui.user_data::<Arc<RefCell<App>>>().unwrap().clone();
    let docker = app.borrow().docker.clone();
    let mut row_index = 0;

    // Fetch currently selected item and perform action based on submitted menu item.
    ui.call_on_name("images_table", |table: &mut ImagesTable| {
        row_index = table.row().unwrap();
        let row = table
            .borrow_item(table.item().expect("No image has beed selected."))
            .unwrap();
        info!("ITEM: {:?}", row.id);

        // Trigger an action based on user menu chosen item.
        match value {
            ImageTableRowAction::Delete => {
                app.borrow_mut()
                    .runtime
                    .block_on(images::delete_image(docker, &row.name))
                    .expect("Couldn't delete the image.");
            }
        };
    });

    // Clear ESC callback.
    ui.clear_global_callbacks(Key::Esc);
    // Recrate the table.
    fetch_table_data(ui, "images_table", Some(row_index));
}
