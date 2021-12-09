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
        html! {
            <>
                <h1>
                   <a href="/">{"gpx.thermokar.st"}</a>
                </h1>

                <p>
                    {"This client-side tool is for merging "}
                    <a href="https://www.topografix.com/gpx.asp">{"GPX files"}</a>
                    {". "}
                </p>

                <Loader />

                <hr/>

                <p>
                    <small>{"source (public access): git://pingo.thermokar.st/gpx-web-utils"}</small>
                    <small>
                        {"Please note, this has only been tested on GPX files produced by "}
                        <a href="https://www.garmin.com">{"Garmin"}</a>
                        {" and "}
                        <a href="https://www.strava.com">{"Strava"}</a>
                        {" - your mileage may vary."}
                    </small>
                </p>
            </>
        }
    }
}
