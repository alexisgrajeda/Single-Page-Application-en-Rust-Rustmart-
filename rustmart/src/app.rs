use serde_json;
use yew::prelude::*;

use crate::components::Navbar;
use crate::pages::{Home, ProductDetail};
use crate::types::{CartProduct, Product};

pub struct App {
    cart_products: Vec<CartProduct>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddToCart(Product),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cart_products = load_cart();
        Self {
            cart_products,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddToCart(product) => {
                if let Some(cart_product) = self
                    .cart_products
                    .iter_mut()
                    .find(|cp| cp.product.id == product.id)
                {
                    cart_product.quantity += 1;
                } else {
                    self.cart_products.push(CartProduct {
                        product,
                        quantity: 1,
                    });
                }
                save_cart(&self.cart_products);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cart_products = self.cart_products.clone();
        let handle_add_to_cart = self.link.callback(Msg::AddToCart);

        let path = yew::utils::window()
            .location()
            .pathname()
            .unwrap_or_default();

        if let Some(id_str) = path.strip_prefix("/product/") {
            if let Ok(id) = id_str.parse::<i32>() {
                return html! {
                    <>
                        <Navbar cart_products={cart_products.clone()} />
                        <ProductDetail id={id} on_add_to_cart={handle_add_to_cart.clone()} />
                    </>
                };
            }
        }

        html! {
            <>
                <Navbar cart_products={cart_products.clone()} />
                <Home cart_products={cart_products} on_add_to_cart={handle_add_to_cart} />
            </>
        }
    }
}

fn load_cart() -> Vec<CartProduct> {
    if let Ok(Some(storage)) = yew::utils::window().local_storage() {
        if let Ok(Some(data)) = storage.get_item("cart") {
            if let Ok(cart) = serde_json::from_str(&data) {
                return cart;
            }
        }
    }
    vec![]
}

fn save_cart(cart: &[CartProduct]) {
    if let Ok(Some(storage)) = yew::utils::window().local_storage() {
        if let Ok(data) = serde_json::to_string(cart) {
            let _ = storage.set_item("cart", &data);
        }
    }
}