use std::collections::HashMap;

use gloo_file::callbacks::FileReader;
use gloo_file::File;
use web_sys::{Blob, Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};

use super::gpx;

pub enum Msg {
    FileLoaded(String, String),
    StartLoad(Vec<File>),
    FilesLoaded,
    Download(Blob),
    Reset,
}

pub struct Loader {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    count: usize,
    is_loading: bool,
    // This field is to handle resetting the native HTML widget's state on error
    field_value: &'static str,
}

impl Component for Loader {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
            count: 0,
            is_loading: false,
            field_value: "",
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

            Msg::StartLoad(files) => {
                self.count = files.len();
                if self.count < 2 {
                    gpx::alert("must load two or more files");
                    ctx.link().send_message(Msg::Reset);
                    return true;
                }
                self.is_loading = true;

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
                let link = ctx.link();

                let merged = match gpx::merge(&self.files) {
                    Ok(result) => result,
                    Err(err) => {
                        gpx::alert(&err.to_string());
                        link.send_message(Msg::Reset);
                        return true;
                    }
                };

                link.send_message(Msg::Download(merged));

                true
            }

            Msg::Download(merged) => {
                let link = ctx.link();

                match gpx::download(merged) {
                    Ok(_) => (),
                    Err(err) => {
                        gpx::alert(&err.to_string());
                        link.send_message(Msg::Reset);
                        return true;
                    }
                }
                link.send_message(Msg::Reset);

                true
            }

            Msg::Reset => {
                self.readers = HashMap::default();
                self.files = vec![];
                self.count = 0;
                self.is_loading = false;
                self.field_value = "";

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = move |e: Event| {
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
            Msg::StartLoad(result)
        };

        html! {
            if self.is_loading {
                <span><strong>{"processing..."}</strong></span>
            } else {
                <input
                    type="file"
                    value={self.field_value}
                    multiple=true
                    onchange={ctx.link().callback(cb)}
                />
            }
        }
    }
}
