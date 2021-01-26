use yew::services::reader::{File};
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, Clamped};
use std::rc::Rc;
use image::{RgbaImage};
use image::imageops;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    LoadFile(Vec<File>),
    Shrink
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadFile(files) => {
                let file = &files[0];
                    let file_url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                    let document = web_sys::window().unwrap().document().unwrap();
                    let image = Rc::new(document
                        .create_element("img").unwrap()
                        .dyn_into::<web_sys::HtmlImageElement>().unwrap()
                    );
                    image.set_src(&file_url);


                    let image_clone = image.clone();

                    let callback = Closure::wrap(Box::new(move || {
                        let canvas = document.get_element_by_id("preview").unwrap();
                        let canvas: web_sys::HtmlCanvasElement = canvas
                            .dyn_into::<web_sys::HtmlCanvasElement>()
                            .map_err(|_| ())
                            .unwrap();

                        let context = canvas
                            .get_context("2d")
                            .unwrap()
                            .unwrap()
                            .dyn_into::<web_sys::CanvasRenderingContext2d>()
                            .unwrap();
                        canvas.set_width(image_clone.natural_width());
                        canvas.set_height(image_clone.natural_height());
                        context.draw_image_with_html_image_element(&image_clone, 0.0, 0.0).unwrap();
                    }) as Box<dyn Fn()>);
                    // .as_ref().unchecked_ref() can extract the &Function from the &JsValue
                    image.set_onload(Some(callback.as_ref().unchecked_ref()));
                    // Do not drop the Closure so the JS callback won't be invalidated after the
                    // function exits
                    callback.forget();
            }
            Msg::Shrink => { 
                // DO nothing yet
                let document = web_sys::window().unwrap().document().unwrap();
                let canvas = document.get_element_by_id("preview").unwrap();
                let canvas: web_sys::HtmlCanvasElement = canvas
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .map_err(|_| ())
                    .unwrap();

                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();
                let width: u32 = canvas.width();
                let height: u32 = canvas.height();
                let image_buffer = context.get_image_data(0.0, 0.0, width.into(), height.into())
                    .unwrap()
                    .data();
                let image: RgbaImage =
                    image::ImageBuffer::from_vec(width, height, image_buffer.to_vec()).unwrap();
                let output_image = imageops::resize(&image, width / 2, height / 2, imageops::FilterType::Nearest);
                let output_image_data = web_sys::ImageData::new_with_u8_clamped_array(Clamped(&mut output_image.into_vec()), width / 2).unwrap();
                context.clear_rect(0.0, 0.0, width.into(), height.into());
                canvas.set_width(width / 2);
                canvas.set_height(height / 2);
                context.put_image_data(&output_image_data, 0.0, 0.0).unwrap();
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input type="file" name="image-upload" id="image-upload" value="" onchange=self.link.callback(move |value| {
                    let mut result = Vec::new();
                    if let ChangeData::Files(files) = value {
                        let files = js_sys::try_iter(&files)
                            .unwrap()
                            .unwrap()
                            .into_iter()
                            .map(|v| File::from(v.unwrap()));
                        result.extend(files);
                    }
                    Msg::LoadFile(result)
                }) />
                <br />
                <button id="shrink" onclick=self.link.callback(move |_| { Msg::Shrink })>{ "Shrink" }</button>
                <br / >
                <canvas id="preview"></canvas>
            </div>
        }
    }
}
