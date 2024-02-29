const fs = require("fs");
const { make_table } = require("./pkg/table_generator.js");
global.getData = function getData() {
  const filePath = "./table_task_files/data.json";
  try {
    const data = fs.readFileSync(filePath, "utf8");
    return JSON.stringify(JSON.parse(data));
  } catch (err) {
    console.error("Error reading JSON file:", err);
    return "";
  }
};

global.writeData = function writeData(tableData) {
  console.log(tableData);
  try {
    fs.writeFileSync("./table_task_files/output/output.json", tableData);
    console.log(`String data has been written to output.json successfully.`);
  } catch (err) {
    console.error(`Error occurred while writing to output.json: ${err}`);
  }
};

make_table();
