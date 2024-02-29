use serde::{Deserialize, Serialize};

use super::font_sizing::calc_font_width;
#[derive(Debug, Deserialize, Serialize)]
pub enum RowTypes {
    HeaderRow,
    DataRow,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    pub height: f32,
    pub page_width: f32,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(rows: Vec<Row>, page_width: f32) -> Table {
        let mut table = Table {
            rows,
            height: 0.0,
            page_width,
        };
        table = table.calculate_height();
        table
    }

    pub fn calculate_height(mut self) -> Table {
        let mut total_height: f32 = 0.0;
        for row in &mut self.rows {
            total_height += row.height;
        }
        self.height = total_height;
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub height: f32,
    pub cell_width: f32,
    #[serde(rename = "rowType")]
    pub row_type: RowTypes,
    pub cells: Vec<Cell>,
}
impl Row {
    pub fn new(cells: Vec<Cell>, row_type: RowTypes, height: f32, cell_width: f32) -> Row {
        Row {
            height,
            cells,
            row_type,
            cell_width,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    pub content: String,
}
impl Cell {
    pub fn new(content: String, font_size: f32, width: f32) -> (Cell, f32) {
        let data = Cell::insert_content(content, font_size, width);
        (Cell { content: data.1 }, data.0)
    }
    pub fn insert_content(content: String, font_size: f32, width: f32) -> (f32, String) {
        // space between cell and content
        let padding = 2.0;
        //space between two lines
        let line_margin = 2.0;
        // remaining space after padding
        let mut remaining_width = width - (2.0 * padding);
        //height of cell
        let mut height = font_size + (2.0 * padding);
        // empty line to push chars
        let mut current_line = String::new();
        // line vec  to push lines
        let mut lines = Vec::new();

        for char in content.chars() {
            let char_width = calc_font_width(char, font_size);
            if char_width <= remaining_width {
                current_line.push(char);
                remaining_width -= char_width;
            } else {
                lines.push(current_line.clone());
                current_line.clear();
                height += font_size + line_margin;
                remaining_width = width - (2.0 * padding);
                current_line.push(char);
                remaining_width -= char_width;
            }
        }
        lines.push(current_line);
        // // Set cell content to the mapped lines
        (height, lines.join("\n"))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderRow {
    #[serde(rename = "fontSize")]
    pub font_size: i32,
    pub title: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DataRow {
    #[serde(rename = "fontSize")]
    pub font_size: i32,
    pub rows: Vec<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "pageWidth")]
    pub page_width: i32,
    #[serde(rename = "headerRow")]
    pub header_row: HeaderRow,
    #[serde(rename = "dataRows")]
    pub data_row: DataRow,
}
