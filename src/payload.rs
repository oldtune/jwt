use serde::Serialize;

#[derive(Serialize)]
pub struct Payload {
    pub claims: Vec<Claim>,
}

#[derive(Serialize)]
pub struct Claim {
    pub name: String,
    pub value: String,
}
