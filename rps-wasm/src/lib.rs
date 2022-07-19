mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert(&rps_core::test_str());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test = rps_core::test_str();
        assert_eq!(test, "testtest".to_string());
    }
}
