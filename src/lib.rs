mod utils;

// use wasm_bindgen::prelude::*;
// 
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// 
// #[wasm_bindgen]
// pub fn merge(files: js_sys::Array) -> wasm_bindgen::JsValue {
//     utils::set_panic_hook();
//     let files: Vec<String> = utils::translate_js_to_rust(files);
//     let merged: gpx::Gpx = utils::join_gpx_files(files);
//     let out_vec: Vec<u8> = utils::write_gpx_to_buffer(merged);
// 
//     JsValue::from_str(&String::from_utf8(out_vec).unwrap())
// }
