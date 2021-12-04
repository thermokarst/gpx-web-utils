use web_sys::Blob;

fn join_gpx_files(files: &Vec<String>) -> gpx::Gpx {
    let mut merged_gpx: gpx::Gpx = Default::default();
    let mut merged_track: gpx::Track = gpx::Track::new();

    for file in files.iter() {
        let buffer = std::io::BufReader::new(file.as_bytes());
        let mut parsed_gpx: gpx::Gpx = gpx::read(buffer).expect("invalid gpx");

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

    merged_gpx
}
fn write_gpx_to_buffer(gpx: gpx::Gpx) -> js_sys::Array {
    let mut buffer = Vec::new();
    gpx::write(&gpx, &mut buffer).unwrap();

    let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&buffer) }.into());
    let array = js_sys::Array::new();
    array.push(&uint8arr.buffer());

    array
}

pub fn merge(files: &Vec<String>) -> Blob {
    let merged: gpx::Gpx = join_gpx_files(files);
    let out_vec = write_gpx_to_buffer(merged);

    Blob::new_with_u8_array_sequence(&out_vec).unwrap()
}
