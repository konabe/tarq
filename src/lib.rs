use swc_ecma_ast::Stmt;
use swc_ecma_ast::{ModuleItem};
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::{SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};


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
    let cm: Lrc<SourceMap> = Default::default();
    // テストのときはこれを使う
    // let fm = cm.new_source_file(
    //     FileName::Custom("index.js".into()).into(),
    //     "function foo() {}",
    // );
    let fm = cm
    .load_file(Path::new("index.js"))
    .expect("failed to load test.js");
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