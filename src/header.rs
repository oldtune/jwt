use serde::Serialize;

#[derive(Serialize)]
pub struct Header {
    pub typ: String,
    pub alg: Algorithm,
}

#[derive(Serialize)]
pub enum Algorithm {
    None,
    HS256,
}
