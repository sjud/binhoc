use std::io;
use std::net::{SocketAddr, TcpListener};
use std::pin::Pin;
use std::str::FromStr;
use axum::body::{Bytes, StreamBody};
use axum::response::{Html as RespHtml, IntoResponse, Response};
use axum::Router;
use axum::routing::{get, get_service,post};
use futures::{stream, Stream, StreamExt, TryStreamExt};
use axum::BoxError;
<<<<<<< HEAD
use axum::handler::{Handler, HandlerWithoutStateExt};
use axum::http::StatusCode;
use yew_ssr::{binhoc_server_data::data,App};
=======
use axum::http::StatusCode;
use yew_ssr::{data,App};
>>>>>>> 58f52867a620df5421562c61bb48117975dec2d9
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
<<<<<<< HEAD
    let router = Router::with_state(true)
=======

    let router = Router::new()
>>>>>>> 58f52867a620df5421562c61bb48117975dec2d9
        .merge(axum_extra::routing::SpaRouter::new("/static", "static"))
        .route("/data",post(data))
        .fallback(index)
        .layer(TraceLayer::new_for_http());

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

<<<<<<< HEAD
async fn process_data(data:String) -> String {
    format!("Data Processed: {}", data)
}
=======
>>>>>>> 58f52867a620df5421562c61bb48117975dec2d9
async fn render(
    index_html_before: String,
    index_html_after: String,
) -> impl Stream<Item = Result<Bytes, BoxError>> + Send {
    let renderer = yew::ServerRenderer::<App>::new();
    stream::once(async move {index_html_before})
        .chain(renderer.render_stream())
        .chain(stream::once(async move{index_html_after}))
        .map(|m|Ok(m.into()))
}

#[axum_macros::debug_handler]
pub async fn index() -> Response {
    let index_html_s = tokio::fs::read_to_string("static/index.html")
        .await
        .expect("failed to read index.html");
    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();
    let renderer = yew::ServerRenderer::<App>::new();

    StreamBody::new(
        stream::once(async move {index_html_before})
            .chain(renderer.render_stream())
            .chain(stream::once(async move{index_html_after}))
            .map(|m|Result::<_,BoxError>::Ok(m))
    ).into_response()

}





