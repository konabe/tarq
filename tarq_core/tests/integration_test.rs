extern crate tarq_core;
use tarq_core::analyze;

#[test]
fn integration_test() {
    let file_name = "index.js";
    let source_code = r#"
        class MyClass {
            constructor() {
                console.log("Hello, World!");
            }
        }
        
        function myFunction() {
            return "This is a function.";
        }
        class AnotherClass {
            method() {
                return "This is another class.";
            }
        }
    "#;
    let output = analyze(file_name, source_code);
    assert_eq!(output, "{\n  \"class_number\": 2,\n  \"function_number\": 1\n}");
}

#[test]
fn integration_test_with_empty() {
    let file_name = "index.js";
    let source_code = r#"
    "#;
    let output = analyze(file_name, source_code);
    assert_eq!(output, "{\n  \"class_number\": 0,\n  \"function_number\": 0\n}");
}
