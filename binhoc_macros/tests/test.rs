use std::net::{SocketAddr, TcpListener};
use axum::body::HttpBody;
use axum::extract::{FromRequest, State};
use axum::{headers, Router, TypedHeader};
use axum::routing::get;
use bincode::{Decode, Encode};
use http::{HeaderMap, HeaderValue, Request, StatusCode};
use reqwest::Client;
use binhoc_macros::binhoc;
use binhoc::{BinHoc1, BinHoc3};
use axum::routing::post;
use http::header::HeaderName;
use crate::headers::{Error, Header};
#[binhoc("/adhoc")]
pub async fn adhoc(
    BinHoc3(email, password, code): BinHoc3<String,String,i32>
) -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc() {
    let router = Router::new()
        .route("/adhoc", post(adhoc));
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
    let password = String::from("pass");
    let code = 32;
    let base = format!("http://{}",addr);
    assert_eq!(
        binhoc_client_adhoc::adhoc
            (&client,base,binhoc_client_adhoc::Vars{email,password,code}).await.unwrap().status(),
        StatusCode::OK
    );
}
#[binhoc("/sadness")]
pub async fn adhoc_sad_route(
    BinHoc3(email, password, code): BinHoc3<String,String,i32>
) -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc_sad_route() {
    let router = Router::new()
        .route("/not_sadness",
               post(adhoc_sad_route));
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
    let password = String::from("pass");
    let code = 32;
    let base = format!("http://{}",addr);
    assert_eq!(
        binhoc_client_adhoc_sad_route
        ::adhoc_sad_route
            (&client,base,binhoc_client_adhoc_sad_route::Vars{email,password,code}).await.unwrap().status(),
        StatusCode::NOT_FOUND
    );
}
#[derive(Clone,Debug)]
pub struct AdhocTestState(String,String,i32);

#[binhoc("/")]
pub async fn adhoc_body_is_state(
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
    )).route("/", post(adhoc_body_is_state));

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
    let password = String::from("pass");
    let code = 32;
    let base = format!("http://{}",addr);
    assert_eq!(
        binhoc_client_adhoc_body_is_state
        ::adhoc_body_is_state
            (&client,base,binhoc_client_adhoc_body_is_state::Vars{email,password,code}).await.unwrap().status(),
        StatusCode::OK
    );
}
#[binhoc("/")]
pub async fn adhoc_optional_arg(
    BinHoc1(is_some):BinHoc1<Option<bool>>
) -> StatusCode {
    assert!(is_some.is_some());
    StatusCode::OK
}
#[tokio::test]
async fn test_adhoc_optional_arg() {
    use crate::AdhocTestState;
    let router = Router::new()
        .route("/", post(adhoc_optional_arg));

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
        binhoc_client_adhoc_optional_arg
        ::adhoc_optional_arg
            (&client,base,binhoc_client_adhoc_optional_arg::Vars{is_some:Some(true)}).await.unwrap().status(),
        StatusCode::OK
    );
}
#[binhoc("/")]
pub async fn adhoc_optional_arg_2(
    BinHoc1(is_none):BinHoc1<Option<bool>>
) -> StatusCode {
    assert!(is_none.is_none());
    StatusCode::OK
}
#[tokio::test]
async fn test_adhoc_optional_arg_2() {
    use crate::AdhocTestState;
    let router = Router::new()
        .route("/", post(adhoc_optional_arg_2));

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
        binhoc_client_adhoc_optional_arg_2
        ::adhoc_optional_arg_2
            (&client,base,binhoc_client_adhoc_optional_arg_2::Vars{is_none:None}).await.unwrap().status(),
        StatusCode::OK
    );
}
#[derive(Encode,Decode)]
pub struct User{
    pub email:String,
    password:String,
    id:u32,
}

#[binhoc("/")]
pub async fn adhoc_with_struct(
    BinHoc1(user):BinHoc1<User>
) -> StatusCode {
    assert_eq!(user.email,"hello".to_string());
    StatusCode::OK
}

#[tokio::test]
async fn test_adhoc_with_struct() {
    let router = Router::new()
        .route("/", post(adhoc_with_struct));

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
        binhoc_client_adhoc_with_struct
        ::adhoc_with_struct
            (&client,base,binhoc_client_adhoc_with_struct::Vars{user:User{
                email: "hello".to_string(),
                password: "".to_string(),
                id: 0
            }}).await.unwrap().status(),
        StatusCode::OK
    );
}

#[binhoc("/")]
pub async fn adhoc_vec_tuple(
    BinHoc1(vec):BinHoc1<Vec<(bool,bool)>>
) -> StatusCode {
    for (b,b1) in vec.into_iter() {
        assert!(b && b1);
    }
    StatusCode::OK
}


#[tokio::test]
async fn test_adhoc_with_vec_tuple() {
    let router = Router::new()
        .route("/", post(adhoc_vec_tuple));

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
        binhoc_client_adhoc_vec_tuple
        ::adhoc_vec_tuple
            (&client,base,binhoc_client_adhoc_vec_tuple::Vars{vec:vec![(true,true),(true,true),(true,true)]})
            .await.unwrap().status(),
        StatusCode::OK
    );
}

static XHEAD : HeaderName = HeaderName::from_static("x-head");
#[derive(Debug)]
pub struct XHead(String);
impl Header for XHead{
    fn name() -> &'static HeaderName {
        &XHEAD
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
        where
            Self: Sized,
            I: Iterator<Item=&'i HeaderValue> {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?;
        Ok(Self(value.to_str().unwrap().to_string()))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let value = HeaderValue::from_str(&self.0).unwrap();
        values.extend(std::iter::once(value));
    }
}

#[binhoc("/")]
pub async fn adhoc_with_header(
    TypedHeader(x_head):TypedHeader<XHead>,
    State(_):State<bool>,
) -> StatusCode {
    assert_eq!(x_head.0,String::from("x-heady"));
    StatusCode::OK
}


#[tokio::test]
async fn test_adhoc_with_headerse() {
    let router = Router::with_state(true)
        .route("/", post(adhoc_with_header));

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
        binhoc_client_adhoc_with_header
        ::adhoc_with_header
            (&client,base,binhoc_client_adhoc_with_header::Vars{x_head:XHead(String::from("x-heady"))})
            .await.unwrap().status(),
        StatusCode::OK
    );
}
use bincode::serde::Compat;
use uuid::Uuid;
#[binhoc("/")]
pub async fn adhoc_with_uuid(
    BinHoc1(uuid):BinHoc1<Compat<Uuid>>,
) -> StatusCode {
    StatusCode::OK
}


#[tokio::test]
async fn test_adhoc_with_uuid() {
    let router = Router::with_state(true)
        .route("/", post(adhoc_with_uuid));

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
        binhoc_client_adhoc_with_uuid
        ::adhoc_with_uuid
            (&client,base,binhoc_client_adhoc_with_uuid::Vars{uuid:Compat(Uuid::new_v4())})
            .await.unwrap().status(),
        StatusCode::OK
    );
}

//    State(state):State<bool>,
#[binhoc("/")]
pub async fn adhoc_with_three(
    TypedHeader(header):TypedHeader<XHead>,
    BinHoc1(uuid):BinHoc1<Compat<Uuid>>,
) -> StatusCode {
    StatusCode::OK
}


#[tokio::test]
async fn test_adhoc_with_three() {
    let router = Router::with_state(true)
        .route("/", post(adhoc_with_three));

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
        binhoc_client_adhoc_with_three
        ::adhoc_with_three
            (&client,base,
             binhoc_client_adhoc_with_three::Vars{
                 header:
             XHead(String::from("x-heady")),
                 uuid: Compat(Uuid::new_v4())})
            .await.unwrap().status(),
        StatusCode::OK
    );
}


