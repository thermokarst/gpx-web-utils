use super::loader::Loader;

use yew::{html, Component, Context, Html};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let topografix = move || -> Html {
            html! { <a href="https://www.topografix.com/gpx.asp">{"GPX files"}</a> }
        };

        let garmin = move || -> Html {
            html! {<a href="https://www.garmin.com">{"Garmin"}</a>}
        };

        let strava = move || -> Html {
            html! {<a href="https://www.strava.com">{"Strava"}</a>}
        };

        html! {
            <>
                <h1>
                   <a href="/">{ "gpx.thermokar.st" }</a>
                </h1>

                <p>
                    { "A client-side tool for merging "}{ topografix() }{ "." }
                </p>

                <Loader />

                <hr/>

                <p>
                    { "Note, this has only been tested on GPX files produced by " }
                    { garmin() }{ " and " }{ strava() }{ " - your mileage may vary." }
                </p>

                <p>
                    <small>
                        { "source (public access): " }
                        { "git://pingo.thermokar.st/gpx-web-utils" }
                    </small>
                </p>
            </>
        }
    }
}
