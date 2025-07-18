use wasm_bindgen::prelude::*;

#[wasm_bindgen]
unsafe extern "C" {
    pub unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe { alert(&format!("Hello, {}!", name)) };
}
