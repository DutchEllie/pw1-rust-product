use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
  html! {
    <>
  <header>
    <h1>{"Portfolio"}</h1>
    </header>
  <nav>
    <h2>{"Navigation"}</h2>
    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
    <Link<Route> to={Route::Test}>{"Test"}</Link<Route>>
  </nav>
  </>
    }
}
