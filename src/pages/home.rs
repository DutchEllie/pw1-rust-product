use yew::prelude::*;
use crate::components::aur::AurViewer;

pub struct Home {
  counter: i32,
}

impl Component for Home {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self { counter: 0 }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <>
      <section>
        {"Hello World"}
        {self.counter}
        <AurViewer package_name={"osu-lazer-bin"}/>
      </section>
      </>
    }
  }
}
