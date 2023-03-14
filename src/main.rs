mod app;

// use app::App;
mod components;
mod pages;

use components::navbar::Navbar;
use pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Home,
  #[at("/test")]
  Test,
}

#[function_component]
fn App() -> Html {
  html! {
    <>
    <BrowserRouter>
      <Navbar/>
      <Switch<Route> render={switch}/>
    </BrowserRouter>
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => {
      html! {
      <main>
        <Home />
      </main>
      }
    }
    Route::Test => {
      html! { <div>{"hello world"}</div>}
    }
  }
}
