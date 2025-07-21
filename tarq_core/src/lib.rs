use swc_ecma_ast::Stmt;
use swc_ecma_ast::{ModuleItem};
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use std::fs::{self, File};
use std::io::Write;

use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

#[wasm_bindgen]
unsafe extern "C" {
    pub unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe { alert(&format!("Hello, {}!", name)) };
}

#[derive(Serialize, Debug)]
struct Output {
    class_number: usize,
}

pub fn execute() {
    let file_name = "index.js".to_string();
    let content = fs::read_to_string(&file_name)
        .expect("Something went wrong reading the file");
    let output_text = analyze(&file_name, &content);
    let output_file_name = "output.json".to_string();
    let result = write_json(&output_file_name, &output_text);
    match result {
        Ok(..) => { println!("Analysis finished. [{}]", output_file_name) }
        Err(err) => { println!("Error! : {}", err) }
    }
}

pub fn analyze(file_name: &str, source_code_text: &str) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom(file_name.into()).into(),
        source_code_text.to_string(),
    );
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let _module = parser
        .parse_module()
        .expect("failed to parser module");
    let class_count = _module.body.iter().filter(|item| {
        match item {
            ModuleItem::Stmt(class_decl) => {
                // println!("Class Name: {}", class_decl.ident.sym);
                match class_decl {
                        Stmt::Decl(class_decl) => {
                            match class_decl {
                                swc_ecma_ast::Decl::Class(_) => {
                                    // println!("Class Name: {}", class_decl.ident.sym);
                                    return true;
                                },
                                _ => return false,
                            }
                        },
                        _ => return false,
                }
            },
            _ => return false,
        }
    }).count();

    let output = Output{class_number: class_count};
    return serde_json::to_string_pretty(&output).unwrap();
}

fn write_json(output_file_name: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(output_file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}