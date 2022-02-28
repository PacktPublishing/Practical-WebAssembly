use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./array")]
extern "C" {
    fn topArray() -> f64;
    fn getNumber() -> i32;
    fn lowerCase(str: &str) -> String;
}

#[wasm_bindgen]
pub fn sum_of_square_root() -> f64 {
    let n = getNumber();
    let mut sum = 0;

    for _ in 0..n {
        sum = sum + (topArray().sqrt() as i64);
    }

    sum
}

#[wasm_bindgen]
pub fn some_string_to_share() -> String {
    lowerCase("HEYA! I am ALL CAPS")
}