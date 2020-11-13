//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_merge() {
    // arrange
    let array: js_sys::Array = js_sys::Array::new();
    let file1 = wasm_bindgen::JsValue::from_str(
        "<?xml version='1.0' encoding='utf-8'?>
         <gpx version='1.0' encoding='UTF-8'>
           <trk>
             <name>file1 tracks</name>
             <type>1</type>
             <trkseg>
               <trkpt lat='35.466388' lon='-111.640076'>
                 <ele>2152.8</ele>
                 <time>2020-09-27T15:39:27+00:00</time>
               </trkpt>
             </trkseg>
           </trk>
         </gpx>",
    );
    let file2 = wasm_bindgen::JsValue::from_str(
        "<?xml version='1.0' encoding='utf-8'?>
         <gpx version='1.0' encoding='UTF-8'>
           <trk>
             <name>file2 tracks</name>
             <type>1</type>
             <trkseg>
               <trkpt lat='35.339854' lon='-111.737165'>
                 <ele>2556.8</ele>
                 <time>2020-09-26T19:07:14+00:00</time>
               </trkpt>
             </trkseg>
           </trk>
         </gpx>",
    );
    array.push(&file1);
    array.push(&file2);

    let exp = wasm_bindgen::JsValue::from_str(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<gpx version=\"1.1\" creator=\"https://github.com/georust/gpx\">
  <metadata>
    <name>merged</name>
    <author>
      <link href=\"https://gpx.thermokar.st\" />
    </author>
  </metadata>
  <trk>
    <name>file1 tracks</name>
    <type>1</type>
    <trkseg>
      <trkpt lat=\"35.466388\" lon=\"-111.640076\">
        <ele>2152.8</ele>
        <time>2020-09-27T15:39:27+00:00</time>
      </trkpt>
    </trkseg>
  </trk>
  <trk>
    <name>file2 tracks</name>
    <type>1</type>
    <trkseg>
      <trkpt lat=\"35.339854\" lon=\"-111.737165\">
        <ele>2556.8</ele>
        <time>2020-09-26T19:07:14+00:00</time>
      </trkpt>
    </trkseg>
  </trk>
  <rte />
</gpx>",
    );

    // act
    let obs = gpx_web_utils::merge(array);

    // assert
    assert_eq!(obs, exp);
}

// TODO: https://github.com/rustwasm/wasm-bindgen/issues/2286
// #[wasm_bindgen_test]
// #[should_panic]
// fn invalid_inputs() {
//     let array: js_sys::Array = js_sys::Array::new_with_length(10);
//     let obs = gpx_web_utils::merge(array);
// }
