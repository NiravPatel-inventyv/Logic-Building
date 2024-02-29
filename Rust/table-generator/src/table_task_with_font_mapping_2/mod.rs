use self::structures::{Cell, Data, DataRow, HeaderRow, Row, RowTypes, Table};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
pub mod font_sizing;
pub mod structures;

pub fn table_task_with_font_mapping(str_data: &str) -> String {
    let deserialized_data: Result<Data, serde_json::Error> = serde_json::from_str(&str_data);
    match deserialized_data {
        Ok(data) => {
            let page_width = data.page_width as f32;
            let header_row_items = data.header_row;
            let data_rows_items = data.data_row;
            let cell_width = page_width / header_row_items.title.len() as f32;
            let header_row = insert_header_row(header_row_items, cell_width);
            let mut data_rows: Vec<Row> = insert_data_rows(data_rows_items, cell_width);
            let mut rows: Vec<Row> = Vec::new();
            rows.push(header_row);
            rows.append(&mut data_rows);
            let table = Table::new(rows, page_width);
            let opt_file = serde_json::to_string_pretty(&table);
            opt_file.unwrap()
        }
        Err(_) => {
            println!("failed to deserialize data");
            "failed to do so".to_string()
        }
    }
}

pub fn insert_header_row(header_row: HeaderRow, cell_width: f32) -> Row {
    let mut header_cells: Vec<Cell> = Vec::new();
    let mut row_height: f32 = 0.0;
    for title in header_row.title {
        let cell = Cell::new(title, header_row.font_size as f32, cell_width);
        header_cells.push(cell.0);
        if row_height < cell.1 {
            row_height = cell.1
        }
    }
    let header_row = Row::new(header_cells, RowTypes::HeaderRow, row_height, cell_width);
    header_row
}
pub fn insert_data_rows(data_rows: DataRow, cell_width: f32) -> Vec<Row> {
    let mut data_rows_vec: Vec<Row> = Vec::new();
    for row in data_rows.rows {
        let mut cells: Vec<Cell> = Vec::new();
        let mut row_height: f32 = 0.0;
        for cell_item in row {
            let cell = Cell::new(cell_item, data_rows.font_size as f32, cell_width);
            cells.push(cell.0);
            if row_height < cell.1 {
                row_height = cell.1
            }
        }
        let data_row = Row::new(cells, RowTypes::DataRow, row_height, cell_width);
        data_rows_vec.push(data_row);
    }
    data_rows_vec
}
