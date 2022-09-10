<h1>BinHoc</h1>
<h3>RPC Codegen for Rapid Prototyping of FullStack Rust wasm apps</h3>
![binhawk](https://github.com/sjud/binhoc/blob/main/binhawk.jpeg?raw=true)

Annotate handlers that are in a shared library with the binhoc attribute which takes
on argument that will resolve to the route the handler will be attached to on the server.

```rust
use binhoc::Binhoc3;

#[cfg(not(target_arch = "wasm32"))]
pub use axum::extract::State;

#[binhoc("/adhoc")]
pub async fn login_with_code(
    State(_state):State<SomeRandomState>,
    BinHoc3(email, password, code): BinHoc3<String,String,i32>
) -> StatusCode {
    StatusCode::OK
}
```

BinHocN, where N is the number of arguments where each argument implements BinCode 
`Decode`/`Encode`.
The attribute generates code that looks like the following:
```rust
pub mod binhoc_client_login_with_code {
    use super::*;
    use bincode::{Decode, Encode};
    #[derive(Encode,Decode)]
    struct BinHoc(String,String,i32);
    pub async fn login_with_code<U: std::fmt::Display>(
        client: &reqwest::Client,
        base: U,
        email: String,
        password: String,
        code: i32,
    ) -> Result<reqwest::Response, anyhow::Error> {
        let body = BinHoc(email,password,code);
        let body = bincode::encode_to_vec(body, bincode::config::standard())?;
        Ok(client
            .post(format!("{}/adhoc",base))
            .body(body)
            .send()
            .await?)
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub mod binhoc_server_login_with_code {
    use super::*;
    pub async fn login_with_code( 
        BinHoc3(email, password, code): BinHoc3<String,String,i32>
    ) -> StatusCode {
        StatusCode::OK
    }
}
```
The client code it generates takes `&reqwest::Client` a base_url and it takes the specific
types specified for BinHocN, the variable names of those types given in the deconstruction
pattern i.e ```Binhoc1(var):BinHoc1<SpecificType>``` would be `var:SpecificType`.

This creates one source of code for client/server connection points as well as type checking
for flexible and rapid prototyping.

You can also put Optional types in BinHocN. ```BinHoc1(optional):BinHoc1<Option<SomeSpecificType>>```
