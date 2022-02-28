use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn is_palindrome(input: &str) -> bool {
    let s = input.to_string().to_lowercase();
    s == s.chars().rev().collect::<String>()
}
