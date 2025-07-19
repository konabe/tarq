use wasm_bindgen::prelude::*;
use serde::{Serialize};
use std::fs::File;
use std::io::Write;


#[derive(Serialize, Debug)]
struct Output {
    class_number: usize,
}

#[wasm_bindgen]
unsafe extern "C" {
    pub unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe { alert(&format!("Hello, {}!", name)) };
}

pub fn execute() {
    let output = Output{class_number: 10};
    let output_file_name = String::from("output.json");
    let result = write_json(&output_file_name, output);
    match result {
        Ok(..) => { println!("Json Output Finished. [{}]", output_file_name) }
        Err(err) => { println!("Error! : {}", err) }
    }
}

fn write_json(output_file_name: &str, output: Output) -> std::io::Result<()> {
    let serialized: String = serde_json::to_string_pretty(&output).unwrap();
    let mut file = File::create(output_file_name)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}