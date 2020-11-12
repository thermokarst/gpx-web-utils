pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn translate_js_to_rust(files: js_sys::Array) -> Vec<String> {
    // https://github.com/rustwasm/wasm-bindgen/issues/111
    files.iter().map(|f| f.as_string().unwrap()).collect()
}

pub fn join_gpx_files(files: Vec<String>) -> gpx::Gpx {
    let mut merged: gpx::Gpx = Default::default();

    for file in files.iter() {
        let buffer = std::io::BufReader::new(file.as_bytes());
        let mut parsed_gpx: gpx::Gpx = gpx::read(buffer).expect("invalid gpx");

        merged.tracks.append(&mut parsed_gpx.tracks);
        merged.waypoints.append(&mut parsed_gpx.waypoints);
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

pub fn write_gpx_to_buffer(gpx: gpx::Gpx) -> Vec<u8> {
    let mut buffer = Vec::new();
    gpx::write(&gpx, &mut buffer).unwrap();

    buffer
}
