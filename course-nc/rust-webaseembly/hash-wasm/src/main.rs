
use sha256::digest;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(word: String) -> String {
    digest(input:word)
}
