use crate::types::Product;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask,Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T,Error>>>;
pub type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_products(callback: FetchCallback<Vec<Product>>) -> FetchTask{
    let request = Request::get("/products/products.json")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(request, callback).unwrap()
}

pub fn get_product(id: i32, callback: FetchCallback<Product>) -> FetchTask {
    let request = Request::get(format!("/products/{}.json",id))
    .body(Nothing)
    .unwrap();

    FetchService::fetch(request,callback).unwrap()
}