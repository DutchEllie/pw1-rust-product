use yew::prelude::*;

pub struct Home {
  counter: i32,
}

impl Component for Home {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self { counter: 0 }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <>
      <section>
        {"Hello World"}
        {self.counter}
      </section>
      </>
    }
  }
}
