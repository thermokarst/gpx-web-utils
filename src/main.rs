use yew::prelude::*;

enum Msg {
    AddOne,
    SubOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div>
            <h1>
              <a href="https://gpx.thermokar.st">{"gpx.thermokar.st"}</a>
            </h1>

            <p>
              {"This client-side tool is for merging "}
              <a href="https://www.topografix.com/gpx.asp">{"GPX files"}</a>
              {". Please note, this has only been tested on GPX files produced by "}
              <a href="https://www.garmin.com">{"Garmin"}</a>
              {" and "}
              <a href="https://www.strava.com">{"Strava"}</a>
              {" - your mileage may vary."}
            </p>

            <button onclick=self.link.callback(|_| Msg::SubOne)>{ "-1" }</button>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            <p>{ self.value }</p>

            <hr/>

            <p>
              <small>
                <a href="https://github.com/thermokarst/gpx-web-utils">
                  {"https://github.com/thermokarst/gpx-web-utils"}
                </a>
              </small>
            </p>
          </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
