use std::error::Error;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, MouseEvent, Url};

fn join_gpx_files(files: &[String]) -> Result<gpx::Gpx, Box<dyn Error>> {
    let mut merged_gpx: gpx::Gpx = Default::default();
    let mut merged_track: gpx::Track = gpx::Track::new();

    for file in files.iter() {
        let buffer = std::io::BufReader::new(file.as_bytes());
        let mut parsed_gpx: gpx::Gpx = gpx::read(buffer)?;

        // consolidate all track segements into one single track.
        for track in parsed_gpx.tracks {
            for segment in track.segments {
                merged_track.segments.push(segment);
            }
        }

        merged_gpx.waypoints.append(&mut parsed_gpx.waypoints);
    }

    merged_gpx.tracks.push(merged_track);

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
    merged_gpx.metadata = Some(metadata);
    merged_gpx.version = gpx::GpxVersion::Gpx11;

    Ok(merged_gpx)
}
fn write_gpx_to_buffer(gpx: gpx::Gpx) -> Result<js_sys::Array, Box<dyn Error>> {
    let mut buffer = Vec::new();
    gpx::write(&gpx, &mut buffer)?;

    let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&buffer) }.into());
    let array = js_sys::Array::new();
    array.push(&uint8arr.buffer());

    Ok(array)
}

pub fn merge(files: &[String]) -> Result<Blob, Box<dyn Error>> {
    let merged: gpx::Gpx = join_gpx_files(files)?;
    let out_vec = write_gpx_to_buffer(merged)?;
    let result = Blob::new_with_u8_array_sequence(&out_vec).map_err(|e| e.as_string().unwrap())?;

    Ok(result)
}

pub fn download(merged: Blob) -> Result<(), Box<dyn Error>> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window
        .document()
        .ok_or("should have a document on window")?;

    let err_handler = |e: JsValue| e.as_string().unwrap();
    let anchor_element = document.create_element("a").map_err(err_handler)?;
    let url = Url::create_object_url_with_blob(&merged).map_err(err_handler)?;

    anchor_element
        .set_attribute("href", &url)
        .map_err(err_handler)?;
    anchor_element
        .set_attribute("download", "merged.gpx")
        .map_err(err_handler)?;

    let event = MouseEvent::new("click").map_err(err_handler)?;
    anchor_element.dispatch_event(&event).map_err(err_handler)?;

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
