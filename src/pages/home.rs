use crate::components::aur::AurViewer;
use yew::prelude::*;

pub struct Home {}

impl Component for Home {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <>
      <section>
        <AurViewer package_name={"osu-lazer-bin"}/>
      </section>
      </>
    }
  }
}
