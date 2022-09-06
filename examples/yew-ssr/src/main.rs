use std::net::{SocketAddr, TcpListener};
use axum::response::Html as RespHtml;
use axum::Router;
use axum::routing::get;
use yew::{Callback, FocusEvent, html, Html, use_node_ref, use_state};
use binhoc_core::BinHoc2;
use web_sys::HtmlInputElement;

pub static BASE_URL :&'static str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/data",get(data));

    let listener = TcpListener::bind("127.0.0.1:3000"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(router.into_make_service())
        .await
        .unwrap()
}

pub async fn index() -> RespHtml {
    let index_html = tokio::fs::read_to_string("static/index.html")
        .await
        .expect("failed to read index.html");
    let rendered = yew::ServerRenderer::<App>::new()
        .render().await;
    Html(rendered)

}

#[binhoc_core::binhoc("get","/data")]
pub async fn data(
    BinHoc2(data_1,data_2):BinHoc2<String,i32>
) -> String {
    format!("{} + {}",data_1,data_2)
}


#[function_component(App)]
pub fn app() -> Html {
    let data_1 = use_node_ref();
    let data_2 = use_node_ref();
    let result = use_state(||String::new());
    let req = {
        let data_1 = data_1.cast::<HtmlInputElement>().unwrap().value();
        let data_2 = data_2.cast::<HtmlInputElement>().unwrap().value();
        binhoc_gen_data::data(
            reqwest::Client::new(),
            BASE_URL,
            data_1,
            data_2
        )
    };
    // .prevent_default() is required for custom behavior for on submit buttons on forms.
    let onsubmit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        req.run();
    });
    html! {
        <div>
            <h2>{ *result.clone()}</h2>
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
