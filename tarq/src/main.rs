use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use tarq_core::analyze;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    file_name: String,
}

pub fn main() {
    let args = Args::parse();
    execute(args);
}

fn execute(args: Args) {
    let file_name = args.file_name;
    println!("Target file: [{file_name}]");
    let content = read_file(&file_name);
    let output_text = analyze(&file_name, &content);
    let output_file_name = "output.json".to_string();
    let result = write_file(&output_file_name, &output_text);
    match result {
        Ok(..) => {
            println!("Analysis finished: [{output_file_name}]")
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
