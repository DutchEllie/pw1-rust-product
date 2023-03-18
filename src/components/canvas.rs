use crate::components::pixel::Pixel;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;
use weblog::console_log;
use yew::prelude::*;

pub enum Msg {
  Start,
  Render,
  Nothing,
}

pub struct CanvasThingy {
  canvas_element: NodeRef,
  canvas: Vec<u8>,
  counter: u32,
  callback: Closure<dyn FnMut()>,
}

impl Component for CanvasThingy {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_future(async { Msg::Start });

    let comp_ctx = ctx.link().clone();
    let callback =
      Closure::wrap(Box::new(move || comp_ctx.send_message(Msg::Render)) as Box<dyn FnMut()>);
    Self {
      canvas_element: NodeRef::default(),
      canvas: Vec::default(),
      counter: 0,
      callback,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Start => {
        let element: HtmlCanvasElement = self.canvas_element.cast().unwrap();

        let rect = element.get_bounding_client_rect();
        element.set_height(rect.height() as u32);
        element.set_width(rect.width() as u32);

        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut counter = 0;

        while y < rect.height() as u32 {
          while x < rect.width() as u32 {
            let p = Pixel::new(x as f64, y as f64, 100, 100, 100, 255);

            self.canvas.push(p.rgba[0]); // Red
            self.canvas.push(p.rgba[1]); // Green
            self.canvas.push(p.rgba[2]); // Blue
            self.canvas.push(p.rgba[3]); // Alpha

            x += 1;
            counter += 1;
          }
          y += 1;
          x = 0;
        }

        console_log!("PIxel");
        // console_log!(self.canvas.len());

        ctx.link().send_message(Msg::Render);
        true
      }
      Msg::Render => {
        self.render();
        true
      }
      Msg::Nothing => true,
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <canvas ref = {self.canvas_element.clone()}/>
      </div>
    }
  }
}

impl CanvasThingy {
  fn render(&mut self) {
    let canvas: HtmlCanvasElement = self.canvas_element.cast().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();

    let width = canvas.width();
    let height = canvas.height();

    let mut canvas_pixel_array: Vec<u8> = self.canvas.clone();

    //canvas_pixel_array[counter * 4] =

    // let pain = Clamped(canvas_pixel_array);

    let rect_x = 50;
    let rect_y = 50;
    let rect_w = 100;
    let rect_h = 200;

    let mut counter = 0;
    for y in rect_y..rect_y + rect_h {
      for x in rect_x..rect_x + rect_w {
        let mut pos = y * width + x;
				canvas_pixel_array[pos as usize] = 200;
        counter += 1;
      }
    }

    // let image_data = ImageData::new_with_u8_clamped_array(pain, width);
    let clam = Clamped(&canvas_pixel_array[..]);

    let image_data = ImageData::new_with_u8_clamped_array(clam, width).unwrap();
    // image_data.data
    ctx.put_image_data(&image_data, 0.0, 0.0);

    window()
      .unwrap()
      .request_animation_frame(self.callback.as_ref().unchecked_ref())
      .unwrap();
  }
}
