use std::net::{SocketAddr, TcpListener};
use axum::body::HttpBody;
use axum::extract::{FromRequest, State};
use axum::Router;
use axum::routing::get;
use bincode::{Decode, Encode};
use http::{Request, StatusCode};
use reqwest::Client;
use binhoc_macros::binhoc;
use binhoc_core::{BinHoc1, BinHoc3};

#[binhoc("get","/adhoc")]
async fn adhoc(
    BinHoc3(email, password, code): BinHoc3<String,String,i32>
) -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc() {
    let router = Router::new()
        .route("/adhoc", get(adhoc));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();
    let email = String::from("email");
    let pass = String::from("pass");
    let code = 32;
    let base = format!("http://{}",addr);
    assert_eq!(
        gen_adhoc_adhoc
        ::adhoc
            (&client,base,email,pass,code).await.unwrap().status(),
        StatusCode::OK
    );
}
#[binhoc("get","/sadness")]
async fn adhoc_sad_route(
    BinHoc3(email, password, code): BinHoc3<String,String,i32>
) -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc_sad_route() {
    let router = Router::new()
        .route("/not_sadness",
               get(adhoc_sad_route));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();
    let email = String::from("email");
    let pass = String::from("pass");
    let code = 32;
    let base = format!("http://{}",addr);
    assert_eq!(
        gen_adhoc_adhoc_sad_route
        ::adhoc_sad_route
            (&client,base,email,pass,code).await.unwrap().status(),
        StatusCode::NOT_FOUND
    );
}
#[derive(Clone,Debug)]
pub struct AdhocTestState(String,String,i32);

#[binhoc("get","/")]
async fn adhoc_body_is_state(
    State(state):State<AdhocTestState>,
    BinHoc3(email,password,code):BinHoc3<String,String,i32>
) -> StatusCode {
    assert_eq!((state.0,state.1,state.2),(email,password,code));
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc_body_is_state() {
    use crate::AdhocTestState;
    let router = Router::with_state(AdhocTestState(
        String::from("email"),
        String::from("pass"),
        32
    )).route("/", get(adhoc_body_is_state));

    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();
    let email = String::from("email");
    let pass = String::from("pass");
    let code = 32;
    let base = format!("http://{}",addr);
    assert_eq!(
        gen_adhoc_adhoc_body_is_state
        ::adhoc_body_is_state
            (&client,base,email,pass,code).await.unwrap().status(),
        StatusCode::OK
    );
}
#[binhoc("get","/")]
async fn adhoc_optional_arg(
    BinHoc1(is_some):BinHoc1<Option<bool>>
) -> StatusCode {
    assert!(is_some.is_some());
    StatusCode::OK
}
#[tokio::test]
async fn test_adhoc_optional_arg() {
    use crate::AdhocTestState;
    let router = Router::new()
        .route("/", get(adhoc_optional_arg));

    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    let client = Client::new();
    let base = format!("http://{}",addr);
    assert_eq!(
        gen_adhoc_adhoc_optional_arg
        ::adhoc_optional_arg
            (&client,base,Some(true)).await.unwrap().status(),
        StatusCode::OK
    );
}
#[binhoc("get","/")]
async fn adhoc_optional_arg_2(
    BinHoc1(is_none):BinHoc1<Option<bool>>
) -> StatusCode {
    assert!(is_none.is_none());
    StatusCode::OK
}
#[tokio::test]
async fn test_adhoc_optional_arg_2() {
    use crate::AdhocTestState;
    let router = Router::new()
        .route("/", get(adhoc_optional_arg_2));

    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    let client = Client::new();
    let base = format!("http://{}",addr);
    assert_eq!(
        gen_adhoc_adhoc_optional_arg_2
        ::adhoc_optional_arg_2
            (&client,base,None).await.unwrap().status(),
        StatusCode::OK
    );
}
#[derive(Encode,Decode)]
pub struct User{
    pub email:String,
    password:String,
    id:u32,
}

#[binhoc("get","/")]
async fn adhoc_with_struct(
    BinHoc1(user):BinHoc1<User>
) -> StatusCode {
    assert_eq!(user.email,"hello".to_string());
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc_with_struct() {
    use crate::AdhocTestState;
    let router = Router::new()
        .route("/", get(adhoc_with_struct));

    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    let client = Client::new();
    let base = format!("http://{}",addr);
    assert_eq!(
        gen_adhoc_adhoc_with_struct
        ::adhoc_with_struct
            (&client,base,User{
                email: "hello".to_string(),
                password: "".to_string(),
                id: 0
            }).await.unwrap().status(),
        StatusCode::OK
    );
}
