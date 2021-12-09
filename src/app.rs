use super::loader::Loader;

use web_sys::Node;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    let topografix = move || -> Html {
        html! { <a href="https://www.topografix.com/gpx.asp">{"GPX files"}</a> }
    };

    html! {
        <>
            <h1>
               <a href="/">{ "gpx.thermokar.st" }</a>
            </h1>

            <p>
                { "a client-side tool for merging "}{ topografix() }
            </p>

            <Loader />

            <hr/>

            <Footer />
        </>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    let notes = Vec::from([
        "this has only been tested on GPX files produced by \
        <a href='https://strava.com'>strava</a> and \
        <a href='https://garmin.com'>garmin</a>",
        "all third-party extension info \
        <a href='https://github.com/georust/gpx/issues/8'>\
        is stripped</a>",
        "if the app breaks, try refreshing the page",
        "source (public access): git://pingo.thermokar.st/gpx-web-utils",
        "source (mirror): \
        <a href='https://github.com/thermokarst/gpx-web-utils'>\
        https://github.com/thermokarst/gpx-web-utils</a>",
    ]);

    html! {
        <div>
            <ul>
                { for notes.iter().map(|f| li(f)) }
            </ul>
            <span>
                <small>{ "\u{000A9} matthew ryan dillon, 2021" }</small>
            </span>
        </div>
    }
}

fn li(data: &str) -> Html {
    let li = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("li")
        .unwrap();
    li.set_inner_html(data);
    let node = Node::from(li);

    VNode::VRef(node)
}
