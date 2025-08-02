use serde::Serialize;
use swc_ecma_ast::ModuleItem;
use swc_ecma_ast::{Decl, Stmt};
use wasm_bindgen::prelude::*;

use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};

#[derive(Serialize, Debug)]
struct Output {
    class_number: usize,
    function_number: usize,
}

#[wasm_bindgen]
pub fn analyze(file_name: &str, source_code_text: &str) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom(file_name.into()).into(),
        source_code_text.to_string(),
    );
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("failed to parser module");
    let class_count = module
        .body
        .iter()
        .filter(|item| matches!(item, ModuleItem::Stmt(Stmt::Decl(Decl::Class(_)))))
        .count();
    let function_count = module
        .body
        .iter()
        .filter(|item| matches!(item, ModuleItem::Stmt(Stmt::Decl(Decl::Fn(_)))))
        .count();
    let output = Output {
        class_number: class_count,
        function_number: function_count,
    };
    serde_json::to_string_pretty(&output).unwrap()
}
