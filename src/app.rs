use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, ProductDetail};
use crate::route::Route;

pub struct App {}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let render = Router::render(|switch: Route| match switch {
      Route::ProductDetail(id) => html!(<ProductDetail id=id />),
      Route::HomePage => html!(<Home />),
    });

    html! {
      <Router<Route, ()> render=render />
    }
  }
}
