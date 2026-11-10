use crate::components::AtcButton;
use crate::types::Product;
use yew::events::MouseEvent;
use yew::prelude::*;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let product_id = self.props.product.id;
        let onclick_card = Callback::from(move |_: MouseEvent| {
            let _ = yew::utils::window()
                .location()
                .set_href(&format!("/product/{}", product_id));
        });
        let href = format!("/product/{}", self.props.product.id);
        html! {
          <div class="product_card_container" onclick=onclick_card>
            <a class="product_card_anchor" href={href}>
                 <img class="product_card_image" src={&self.props.product.image}/>
                 <div class="product_card_name">{&self.props.product.name}</div>
                 <div class="product_card_price">{"$"}{&self.props.product.price}</div>
            </a>
            <AtcButton product={self.props.product.clone()} on_add_to_cart={self.props.on_add_to_cart.clone()} />
            </div>
        }
    }
}