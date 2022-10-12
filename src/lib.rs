use regex::Regex;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let message = "Hello, regex-wasm-demo!";
    let greeting_re = Regex::new("(Hello|Hola)").unwrap();
    let greeting_found = greeting_re.find(message).is_some();
    assert!(greeting_found);
    alert(message);
}
