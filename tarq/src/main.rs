use std::fs::{self, File};
use std::io::Write;
use tarq_core::analyze;

fn main() {
    execute();
}

fn execute() {
    let file_name = "index.js".to_string();
    let content = read_file(&file_name);
    let output_text = analyze(&file_name, &content);
    let output_file_name = "output.json".to_string();
    let result = write_file(&output_file_name, &output_text);
    match result {
        Ok(..) => {
            println!("Analysis finished. [{output_file_name}]")
        }
        Err(err) => {
            println!("Error! : {err}")
        }
    }
}

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn write_file(output_file_name: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(output_file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
