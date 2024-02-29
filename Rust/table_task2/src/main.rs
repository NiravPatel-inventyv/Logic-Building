use std::{cell, collections::HashMap, fs};

use serde_json::Value;

#[derive(Debug)]
struct Cell {
    width: f32,
    content: String,
}

impl Cell {
    fn new(width: f32) -> Self {
        Cell {
            width,
            content: String::new(),
        }
    }
}

#[derive(Debug)]
struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn new() -> Self {
        Row { cells: Vec::new() }
    }
}

#[derive(Debug)]
struct Table {
    page_width: f32,
    rows: Vec<Row>,
}

impl Table {
    fn new(page_width: f32) -> Self {
        Table {
            page_width,
            rows: Vec::new(),
        }
    }

    fn insert_header_row(&mut self, titles: Vec<&str>, font_size: f32) {
        let cell_width = self.page_width / 5.0;
        let mut header_row = Row::new();

        for title in titles {
            let mut cell = Cell::new(cell_width);
            self.insert_content(&mut cell, title, font_size);
            header_row.cells.push(cell);
        }

        self.rows.push(header_row);
    }

    fn insert_data_rows(&mut self, rows: &Vec<Value>, font_size: f32) {
        for row in rows {
            let cell_width = self.page_width / 5.0;
            let mut data_row = Row::new();
            for cell_data in row.as_array().unwrap() {
                let mut cell = Cell::new(cell_width);
                self.insert_content(&mut cell, cell_data.as_str().unwrap(), font_size);
                data_row.cells.push(cell);
            }
            self.rows.push(data_row);
        }
       
    }

    fn insert_content(&self, cell: &mut Cell, content: &str, font_size: f32) {
        let padding = 2.0;
        let line_margin = 2.0;
        let remaining_width = cell.width - (2.0 * padding); // Consider padding on both sides

        let mut current_line = String::new();
        let mut lines = Vec::new();

        for char in content.chars() {
            let char_width = calc_font_width(char); // You need to calculate char widths based on font and font size

            if char_width + (current_line.len() as f32 * padding) <= remaining_width {
                current_line.push(char);
            } else {
                lines.push(current_line.clone());
                current_line.clear();
                current_line.push(char);
            }
        }
// println!("{}",current_line)
        lines.push(current_line);

        // // Now adjust lines to fit inside the cell vertically, considering line margins
        let max_lines = (font_size + line_margin) * 2.0;
        if lines.len() as f32> max_lines {
            lines.truncate(max_lines as usize);
        }

        // // Set cell content to the mapped lines
        cell.content = lines.join("\n");
        // println!("{}",cell.content);
    }
}

fn calc_font_width(c: char) -> f32 {
    let mut char_width_map: HashMap<char, f32> = HashMap::new();

    char_width_map.insert('0', 0.5);
    char_width_map.insert('1', 0.5);
    char_width_map.insert('2', 0.5);
    char_width_map.insert('3', 0.5);
    char_width_map.insert('4', 0.5);
    char_width_map.insert('5', 0.5);
    char_width_map.insert('6', 0.5);
    char_width_map.insert('7', 0.5);
    char_width_map.insert('8', 0.5);
    char_width_map.insert('9', 0.5);
    char_width_map.insert(' ', 0.25);
    char_width_map.insert('!', 0.333);
    char_width_map.insert('\"', 0.555);
    char_width_map.insert('#', 0.5);
    char_width_map.insert('$', 0.5);
    char_width_map.insert('%', 1.0);
    char_width_map.insert('&', 0.83300006);
    char_width_map.insert('\'', 0.27800003);
    char_width_map.insert('(', 0.333);
    char_width_map.insert(')', 0.333);
    char_width_map.insert('*', 0.5);
    char_width_map.insert('+', 0.57000005);
    char_width_map.insert(':', 0.25);
    char_width_map.insert('-', 0.333);
    char_width_map.insert('.', 0.25);
    char_width_map.insert('/', 0.27800003);
    char_width_map.insert(',', 0.333);
    char_width_map.insert(';', 0.333);
    char_width_map.insert('<', 0.57000005);
    char_width_map.insert('=', 0.57000005);
    char_width_map.insert('>', 0.57000005);
    char_width_map.insert('?', 0.5);
    char_width_map.insert('@', 0.93000007);
    char_width_map.insert('A', 0.72200006);
    char_width_map.insert('B', 0.66700006);
    char_width_map.insert('C', 0.72200006);
    char_width_map.insert('D', 0.72200006);
    char_width_map.insert('E', 0.66700006);
    char_width_map.insert('F', 0.611);
    char_width_map.insert('G', 0.77800006);
    char_width_map.insert('H', 0.77800006);
    char_width_map.insert('I', 0.38900003);
    char_width_map.insert('J', 0.5);
    char_width_map.insert('K', 0.77800006);
    char_width_map.insert('L', 0.66700006);
    char_width_map.insert('M', 0.94400007);
    char_width_map.insert('N', 0.72200006);
    char_width_map.insert('O', 0.77800006);
    char_width_map.insert('P', 0.611);
    char_width_map.insert('Q', 0.77800006);
    char_width_map.insert('R', 0.72200006);
    char_width_map.insert('S', 0.55600005);
    char_width_map.insert('T', 0.66700006);
    char_width_map.insert('U', 0.72200006);
    char_width_map.insert('V', 0.72200006);
    char_width_map.insert('W', 1.0);
    char_width_map.insert('X', 0.72200006);
    char_width_map.insert('Y', 0.72200006);
    char_width_map.insert('Z', 0.66700006);
    char_width_map.insert('[', 0.333);
    char_width_map.insert('\\', 0.27800003);
    char_width_map.insert(']', 0.333);
    char_width_map.insert('^', 0.58100003);
    char_width_map.insert('_', 0.5);
    char_width_map.insert('`', 0.333);
    char_width_map.insert('a', 0.5);
    char_width_map.insert('b', 0.55600005);
    char_width_map.insert('c', 0.44400004);
    char_width_map.insert('d', 0.55600005);
    char_width_map.insert('e', 0.44400004);
    char_width_map.insert('f', 0.333);
    char_width_map.insert('g', 0.5);
    char_width_map.insert('h', 0.55600005);
    char_width_map.insert('i', 0.27800003);
    char_width_map.insert('j', 0.333);
    char_width_map.insert('k', 0.55600005);
    char_width_map.insert('l', 0.27800003);
    char_width_map.insert('m', 0.83300006);
    char_width_map.insert('n', 0.55600005);
    char_width_map.insert('o', 0.5);
    char_width_map.insert('p', 0.55600005);
    char_width_map.insert('q', 0.55600005);
    char_width_map.insert('r', 0.44400004);
    char_width_map.insert('s', 0.38900003);
    char_width_map.insert('t', 0.333);
    char_width_map.insert('u', 0.55600005);
    char_width_map.insert('v', 0.5);
    char_width_map.insert('w', 0.72200006);
    char_width_map.insert('x', 0.5);
    char_width_map.insert('y', 0.5);
    char_width_map.insert('z', 0.44400004);
    char_width_map.insert('{', 0.39400002);
    char_width_map.insert('|', 0.22000001);
    char_width_map.insert('}', 0.39400002);
    char_width_map.insert('~', 0.52000004);

    let char_width = *char_width_map.get(&c).unwrap();
    char_width
}
fn main() {
    let data = fs::read_to_string("./data.json").unwrap();
    let data: Value = serde_json::from_str(&data).unwrap();
    let data_obj = data.as_object().unwrap();
    let header_row_titles = data_obj
        .get("headerRow")
        .unwrap()
        .as_object()
        .unwrap()
        .get("title")
        .unwrap()
        .as_array()
        .unwrap();
    let head_titles: Vec<&str> = header_row_titles
        .iter()
        .map(|title| title.as_str().unwrap())
        .collect();
    // println!("{:#?}",header_row_titles);
    let data_rows = data_obj
        .get("dataRows")
        .unwrap()
        .as_object()
        .unwrap()
        .get("rows")
        .unwrap()
        .as_array()
        .unwrap();
    // println!("{:#?}",data_rows);

    // for head_row in header_row_titles.iter(){
    //     println!("{}",head_row)
    // }
    //  for data_row in data_rows.iter(){
    //     for data_item in data_row.as_array().unwrap(){
    //         println!("{:#?}",data_item.as_str().unwrap());
    //     }
    //     println!("----")
    // }
    let mut table = Table::new(792.0);
    table.insert_header_row(head_titles, 18.0);

    table.insert_data_rows(data_rows, 18.0);
    println!("{:#?}", table);
}
