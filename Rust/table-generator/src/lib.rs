mod table_task_with_font_mapping_2;
mod utils;
use table_task_with_font_mapping_2::table_task_with_font_mapping;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn getData() -> String;
    fn writeData(tab_data: String);
}

#[wasm_bindgen]
pub fn make_table() {
    let table_data = getData();
    let out_data = table_task_with_font_mapping(&table_data);
    writeData(out_data);
}
