use super::loader::Loader;

use web_sys::Node;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>
                <a href="/">
                    { "gpx.thermokar.st" }
                </a>
            </h1>

            <p>
                { "a client-side tool for merging "}
                <a href="https://www.topografix.com/gpx.asp">
                    { "gpx files" }
                </a>
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
        // note 1
        "this has only been tested on GPX files produced by \
        <a href=\"https://strava.com\" target=\"_blank\">strava</a> and \
        <a href=\"https://garmin.com\" target=\"_blank\">garmin</a>",
        // note 2
        "all third-party extension info \
        <a href=\"https://github.com/georust/gpx/issues/8\" target=\"_blank\">\
        is stripped</a>",
        // note 3
        "if the app breaks, try refreshing the page",
        "source (public access): git://pingo.thermokar.st/gpx-web-utils",
        // note 4
        "source (mirror): \
        <a href=\"https://github.com/thermokarst/gpx-web-utils\" target=\"_blank\">\
        https://github.com/thermokarst/gpx-web-utils</a>",
    ]);

    html! {
        <div>
            <ul>
                { for notes.iter().map(|n| inner_html_enabled_li(n)) }
            </ul>

            <span>
                <small>
                    { "\u{000A9} matthew ryan dillon, 2021" }
                </small>
            </span>
        </div>
    }
}

fn inner_html_enabled_li(data: &str) -> Html {
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
