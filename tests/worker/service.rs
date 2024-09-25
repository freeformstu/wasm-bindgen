#![cfg(target_family = "wasm")]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_service_worker);

pub mod modules;

// should not be executed
#[wasm_bindgen(start)]
fn start() {
    panic!();
}
