use std::collections::HashMap;

use gloo_file::callbacks::FileReader;
use gloo_file::File;
use web_sys::{Event, HtmlInputElement, Url, MouseEvent};
use yew::{html, html::TargetCast, Component, Context, Html};

use super::utils;

pub enum Msg {
    FileLoaded(String, String),
    Files(Vec<File>),
    FilesLoaded,
    Reset,
}

pub struct Loader {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    count: usize,
}

impl Component for Loader {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
            count: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileLoaded(filename, data) => {
                self.files.push(data);
                self.readers.remove(&filename);
                if self.files.len() == self.count {
                    ctx.link().send_message(Msg::FilesLoaded);
                }
                true
            }

            Msg::Files(files) => {
                self.count = files.len();
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_text(&file, move |res| {
                            link.send_message(Msg::FileLoaded(
                                file_name,
                                res.unwrap_or_else(|e| e.to_string()),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }

            Msg::FilesLoaded => {
                let merged = utils::merge(&self.files);

                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let anchor_element = document.create_element("a").unwrap();

                let url = Url::create_object_url_with_blob(&merged).unwrap();

                anchor_element.set_attribute("href", &url).unwrap();
                anchor_element.set_attribute("download", "merged.gpx").unwrap();

                let event = MouseEvent::new("click").unwrap();
                anchor_element.dispatch_event(&event).unwrap();

                true
            }

            Msg::Reset => {
                self.readers = HashMap::default();
                self.files = vec![];
                self.count = 0;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <input type="file" multiple=true onchange={link.callback(move |e: Event| {
                    let mut result = Vec::new();
                    let input: HtmlInputElement = e.target_unchecked_into();

                    if let Some(files) = input.files() {
                        let files = js_sys::try_iter(&files)
                            .unwrap()
                            .unwrap()
                            .map(|v| web_sys::File::from(v.unwrap()))
                            .map(File::from);
                        result.extend(files);
                    }
                    Msg::Files(result)
                    })}
                />
            </>
        }
    }
}
