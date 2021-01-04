use crate::api;
use crate::components::ProductCard;
use crate::types::{CartProduct, Product};
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
  products: Vec<Product>,
  cart_products: Vec<CartProduct>,
  get_products_error: Option<Error>,
  get_products_loaded: bool,
}

pub struct Home {
  state: State,
  link: ComponentLink<Self>,
  task: Option<FetchTask>,
}

pub enum Msg {
  AddToCart(i32),
  GetProducts,
  GetProductsSuccess(Vec<Product>),
  GetProductsError(Error),
}

impl Component for Home {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let products: Vec<Product> = vec![];
    let cart_products = vec![];

    link.send_message(Msg::GetProducts);

    Self {
      state: State {
        products,
        cart_products,
        get_products_error: None,
        get_products_loaded: false,
      },
      link,
      task: None,
    }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Msg::GetProducts => {
        self.state.get_products_loaded = false;
        let handler = self
          .link
          .callback(move |response: api::FetchResponse<Vec<Product>>| {
            let (_, Json(data)) = response.into_parts();

            match data {
              Ok(products) => Msg::GetProductsSuccess(products),
              Err(err) => Msg::GetProductsError(err),
            }
          });
        self.task = Some(api::get_products(handler));
        true
      }
      Msg::GetProductsSuccess(products) => {
        self.state.products = products;
        self.state.get_products_loaded = true;
        true
      }
      Msg::GetProductsError(error) => {
        self.state.get_products_error = Some(error);
        self.state.get_products_loaded = true;
        true
      }
      Msg::AddToCart(product_id) => {
        let product = self
          .state
          .products
          .iter()
          .find(|p: &&Product| p.id == product_id)
          .unwrap();

        let cart_product = self
          .state
          .cart_products
          .iter_mut()
          .find(|cp: &&mut CartProduct| cp.product.id == product_id);

        if let Some(cp) = cart_product {
          cp.quantity += 1;
        } else {
          self.state.cart_products.push(CartProduct {
            product: product.clone(),
            quantity: 1,
          })
        }
        true
      }
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let products: Vec<Html> = self
      .state
      .products
      .iter()
      .map(|product: &Product| {
        let product_id = product.id;
        html! {
          <ProductCard product={product} on_add_to_cart=self.link.callback(move |_| Msg::AddToCart(product_id))/>
        }
      })
      .collect();

    let cart_value = self
      .state
      .cart_products
      .iter()
      .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

    if !self.state.get_products_loaded {
      html! {
        <div>{"Loading ... "}</div>
      }
    } else if let Some(_) = self.state.get_products_error {
      html! {
        <div>
          <span>{"Error loading products! :("}</span>
        </div>
      }
    } else {
      html! {
        <div>
          <span>{ format!("Cart Value: {:.2}", cart_value) }</span>
          <span>{ products }</span>
        </div>
      }
    }
  }
}
