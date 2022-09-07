use std::str::FromStr;
use yew::{Callback, FocusEvent, html, Html, use_node_ref, use_state};
use yew::platform::spawn_local;
use yew::suspense::use_future;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use binhoc_core::BinHoc2;

pub static BASE_URL :&'static str = "http://127.0.0.1:3000";

#[binhoc_core::binhoc("/data")]
pub async fn data(BinHoc2(data_1,data_2):BinHoc2<String,i32>) -> String {
    format!("{} + {}",data_1,data_2)
}
#[function_component]
pub fn App() -> Html {
    let data_1 = use_node_ref();
    let data_2 = use_node_ref();
    let result = use_state(||String::new());
    let data_1_c = data_1.clone();
    let data_2_c = data_2.clone();
    let result_c = result.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let data_1_c = data_1_c.clone();
        let data_2_c = data_2_c.clone();
        let result_c = result_c.clone();
        spawn_local(async move {
            let data_1 = data_1_c.cast::<HtmlInputElement>().unwrap().value();
            let data_2 = data_2_c.cast::<HtmlInputElement>().unwrap().value();
            let data_2 = i32::from_str(&data_2).unwrap();
            let resp : reqwest::Response  = gen_binhoc_data::data(
                &reqwest::Client::new(),
                BASE_URL,
                data_1,
                data_2
            ).await.unwrap();
            result_c.set(
                resp.text().await.unwrap()
            );
        });
    });
    html! {
        <div>
            <h2>{ (*result).clone()}</h2>
            <form {onsubmit}>
                <input type="text" placeholder="Data_1" ref={data_1.clone()}/>
                <br/>
                <input type="text" placeholder="Data_2" ref={data_2.clone()}/>
                <br/>
                <button type="submit" disabled=false>
        <h3>{ "Create Account" }</h3> </button>
            </form>
        </div>
    }
}