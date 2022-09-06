use axum_core::extract::FromRequest;
use bincode::{Decode, Encode};
use hyper::body::HttpBody;
use hyper::{Request, StatusCode};

#[derive(Encode,Decode)]
pub struct BinHoc1<A:Encode+Decode>(pub A);


#[async_trait::async_trait]
impl<State,A,Body> FromRequest<State,Body> for BinHoc1<A>
    where
        State:Sync,
        <Body as HttpBody>::Data:Send,
        Body: HttpBody + Send + 'static,
        A:Encode+Decode, {
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, _: &State) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let a: A = bincode::decode_from_slice(
            &body,
            bincode::config::standard()
        ).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?.0;
        Ok(Self(a))
    }
}

#[derive(Encode,Decode)]
pub struct BinHoc2<
    A:Encode+Decode,
    B:Encode+Decode>
(pub A,pub B,);

#[async_trait::async_trait]
impl<State,A,B,Body> FromRequest<State,Body> for BinHoc2<A,B>
    where
        State:Sync,
        <Body as HttpBody>::Data:Send,
        Body: HttpBody + Send + 'static,
        A:Encode+Decode,
        B:Encode+Decode, {
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, _: &State) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let (a, b): (A, B) = bincode::decode_from_slice(
            &body,
            bincode::config::standard()
        ).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?.0;
        Ok(Self(a, b))
    }
}


        #[derive(Encode,Decode)]
pub struct BinHoc3<
    A:Encode+Decode,
    B:Encode+Decode,
    C:Encode+Decode>
(pub A,pub B,pub C,);

#[async_trait::async_trait]
impl<State,A,B,C,Body> FromRequest<State,Body> for BinHoc3<A,B,C>
    where
        State:Sync,
        <Body as HttpBody>::Data:Send,
        Body: HttpBody + Send + 'static,
        A:Encode+Decode,
        B:Encode+Decode,
        C:Encode+Decode, {
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, _: &State) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
        let (a,b,c) : (A,B,C) = bincode::decode_from_slice(
            &body,
            bincode::config::standard()
        ).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?.0;
        Ok(Self(a,b,c))

    }
}
#[derive(Encode,Decode)]
pub struct BinHoc4<
    A:Encode+Decode,
    B:Encode+Decode,
    C:Encode+Decode,
    D2:Encode+Decode, // D is already used by Decode derive??
>(pub A,pub B,pub C, pub D2);

#[async_trait::async_trait]
impl<State,A,B,C,D2,Body> FromRequest<State,Body> for BinHoc4<A,B,C,D2>
    where
        State:Sync,
        <Body as HttpBody>::Data:Send,
        Body: HttpBody + Send + 'static,
        A:Encode+Decode,
        B:Encode+Decode,
        C:Encode+Decode,
        D2:Encode+Decode,{
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, _: &State) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let (a, b,c,d,): (A, B,C,D2,) = bincode::decode_from_slice(
            &body,
            bincode::config::standard()
        ).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?.0;
        Ok(Self(a, b,c,d,))
    }
}
#[derive(Encode,Decode)]
pub struct BinHoc5<
    A:Encode+Decode,
    B:Encode+Decode,
    C:Encode+Decode,
    D2:Encode+Decode, // D is already used by Decode derive??
    E2:Encode+Decode, // E is already used by Encode derive??
>(pub A,pub B,pub C, pub D2, pub E2,);

#[async_trait::async_trait]
impl<State,A,B,C,D2,E2,Body> FromRequest<State,Body> for BinHoc5<A,B,C,D2,E2>
    where
        State:Sync,
        <Body as HttpBody>::Data:Send,
        Body: HttpBody + Send + 'static,
        A:Encode+Decode,
        B:Encode+Decode,
        C:Encode+Decode,
        D2:Encode+Decode,
        E2:Encode+Decode,{
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, _: &State) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let (a, b,c,d,e): (A, B,C,D2,E2,) = bincode::decode_from_slice(
            &body,
            bincode::config::standard()
        ).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?.0;
        Ok(Self(a, b,c,d,e))
    }
}
#[derive(Encode,Decode)]
pub struct BinHoc6<
    A:Encode+Decode,
    B:Encode+Decode,
    C:Encode+Decode,
    D2:Encode+Decode, // D is already used by Decode derive??
    E2:Encode+Decode, // E is already used by Encode derive??
    F:Encode+Decode
>(pub A,pub B,pub C, pub D2, pub E2, pub F,);

#[async_trait::async_trait]
impl<State,A,B,C,D2,E2,F,Body> FromRequest<State,Body> for BinHoc6<A,B,C,D2,E2,F>
    where
        State:Sync,
        <Body as HttpBody>::Data:Send,
        Body: HttpBody + Send + 'static,
        A:Encode+Decode,
        B:Encode+Decode,
        C:Encode+Decode,
        D2:Encode+Decode,
        E2:Encode+Decode,
        F:Encode+Decode{
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, _: &State) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let (a, b,c,d,e,f): (A, B,C,D2,E2,F) = bincode::decode_from_slice(
            &body,
            bincode::config::standard()
        ).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?.0;
        Ok(Self(a, b,c,d,e,f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn does_this_compile(BinHoc1(is_some):BinHoc1<Option<bool>>) {
        assert!(is_some.unwrap());
    }
    #[test]
    fn test_optional() {
        does_this_compile(BinHoc1(Some(true)))
    }
}