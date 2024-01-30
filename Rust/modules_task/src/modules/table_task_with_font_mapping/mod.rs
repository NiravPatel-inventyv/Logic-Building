use self::font_sizing::calc_font_width;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
pub mod font_sizing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    page_width: f32,
    rows: Vec<Row>,
    height: f32,
}

impl Table {
    fn new(page_width: f32) -> Table {
        Table {
            height: 0.0,
            page_width,
            rows: Vec::new(),
        }
    }
    fn calculate_height(&mut self) {
        for row in self.rows.iter() {
            self.height += row.height
        }
    }
    //insert header row
    fn insert_header_row(&mut self, titles: Vec<&str>, font_size: f32) {
        let cell_width = self.page_width / titles.len() as f32;
        let mut header_row = Row::new();
        for title in titles {
            let mut cell = Cell::new(cell_width);
            self.insert_content(&mut cell, title, font_size);
            header_row.cells.push(cell);
        }
        header_row.calculate_height();
        self.rows.push(header_row);
    }

    // insert data rows
    fn insert_data_rows(&mut self, rows: &Vec<Value>, font_size: f32) {
        for row in rows {
            let cell_width = self.page_width / 16.0;
            let mut data_row = Row::new();
            for cell_data in row.as_array().unwrap() {
                let mut cell = Cell::new(cell_width);
                self.insert_content(&mut cell, cell_data.as_str().unwrap(), font_size);
                data_row.cells.push(cell);
            }
            data_row.calculate_height();
            self.rows.push(data_row);
        }
    }

    fn insert_content(&self, cell: &mut Cell, content: &str, font_size: f32) {
        // space between cell and content
        let padding = 2.0;
        //space between two lines
        let line_margin = 2.0;
        // remaining space after padding
        let mut remaining_width = cell.width - (2.0 * padding);
        //height of cell
        let mut height = 10.0 + (2.0 * padding);
        // empty line to push chars
        let mut current_line = String::new();
        // line vec  to push lines
        let mut lines = Vec::new();

        for char in content.chars() {
            let char_width = calc_font_width(char,font_size);
            if char_width <= remaining_width {
                current_line.push(char);
                remaining_width -= char_width;
            } else {
                lines.push(current_line.clone());
                current_line.clear();
                height = (height + 10.0) + line_margin;
                remaining_width = cell.width - (2.0 * padding);
                current_line.push(char);
                remaining_width -= char_width;
            }
        }
        lines.push(current_line);
        // // Set cell content to the mapped lines
        cell.content = lines.join("\n");
        cell.height = height;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    height: f32,
    cells: Vec<Cell>,
}
impl Row {
    fn new() -> Row {
        Row {
            height: 0.0,
            cells: Vec::new(),
        }
    }
    fn calculate_height(&mut self) {
        let mut max_height: f32 = 0.0;
        for cell in &self.cells {
            if cell.height > max_height {
                max_height = cell.height
            }
        }
        for cell_item in &mut self.cells {
            cell_item.height = max_height
        }
        self.height = max_height
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    height: f32,
    width: f32,
    content: String,
}
impl Cell {
    fn new(width: f32) -> Cell {
        Cell {
            height: 0.0,
            width,
            content: String::new(),
        }
    }
}

pub fn table_task_with_font_mapping() {
    // reading json file to string
    let data = fs::read_to_string("./src/data/table_task_files/data.json").unwrap();

    // deserializing into Value
    let data: Value = serde_json::from_str(&data).unwrap();

    // casting deserialized data into object
    let data_obj = data.as_object().unwrap();

    // extracting header_rows from object
    let head_titles = data_obj
        .get("headerRow")
        .unwrap()
        .as_object()
        .unwrap()
        .get("title")
        .unwrap()
        .as_array()
        .unwrap().iter()
        .map(|title| title.as_str().unwrap())
        .collect();
   
    // extracting data_rows from object
    let data_rows = data_obj
        .get("dataRows")
        .unwrap()
        .as_object()
        .unwrap()
        .get("rows")
        .unwrap()
        .as_array()
        .unwrap();

    let font_size_header = data_obj
        .get("headerRow")
        .unwrap()
        .as_object()
        .unwrap()
        .get("fontSize")
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    let font_size_data = data_obj
        .get("dataRows")
        .unwrap()
        .as_object()
        .unwrap()
        .get("fontSize")
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    let page_width = data_obj.get("pageWidth").unwrap().as_f64().unwrap() as f32;

    let mut table = Table::new(page_width);
    table.insert_header_row(head_titles, font_size_header);
    table.insert_data_rows(data_rows, font_size_data);
    table.calculate_height();
    println!("{:#?}", table);
    let opt_file = serde_json::to_string_pretty(&table).unwrap();
    fs::write("./src/data/table_task_files/output/data.json", &opt_file).unwrap();
    println!("Table printed succesfully");
}
