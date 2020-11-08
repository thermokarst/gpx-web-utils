mod utils;

use js_sys::Array;
use std::io::BufReader;
use std::str;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: JsValue);
}

#[wasm_bindgen]
pub fn merge(files: Array) -> JsValue {
    utils::set_panic_hook();

    let files = translate_js_to_rust(files);
    let merged: gpx::Gpx = join_gpx_files(files);
    let out_vec = write_gpx_to_buffer(merged);
    JsValue::from_str(&String::from_utf8(out_vec).unwrap())
}

fn translate_js_to_rust(files: Array) -> Vec<String> {
    // I am sure there is a better way to do this, but wasm_bindgen doesn't
    // play well with Vec<T> (or Vec<Vec<T>>):
    // https://github.com/rustwasm/wasm-bindgen/issues/111
    files.iter().map(|f| f.as_string().unwrap()).collect()
}

fn join_gpx_files(files: Vec<String>) -> gpx::Gpx {
    let mut merged: gpx::Gpx = Default::default();

    for file in files.iter() {
        let buffer = BufReader::new(file.as_bytes());
        let parsed_gpx: gpx::Gpx = gpx::read(buffer).expect("oops1");

        for track in parsed_gpx.tracks.into_iter() {
            merged.tracks.push(track);
        }

        for waypoint in parsed_gpx.waypoints.into_iter() {
            merged.waypoints.push(waypoint);
        }
    }

    let link = gpx::Link {
        href: String::from("https://gpx.thermokar.st"),
        ..Default::default()
    };
    let author = gpx::Person {
        link: Some(link),
        ..Default::default()
    };
    let metadata = gpx::Metadata {
        name: Some(String::from("merged")),
        author: Some(author),
        ..Default::default()
    };
    merged.metadata = Some(metadata);
    merged.version = gpx::GpxVersion::Gpx11;

    merged
}

fn write_gpx_to_buffer(gpx: gpx::Gpx) -> Vec<u8> {
    let mut buffer = Vec::new();
    gpx::write(&gpx, &mut buffer).unwrap();
    buffer
}
