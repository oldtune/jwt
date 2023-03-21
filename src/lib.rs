use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use header::Header;
use payload::Payload;

pub mod header;
pub mod payload;
pub mod signature;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let header = super::header::Header {
            alg: header::Algorithm::HS256,
            typ: "JWT".to_string(),
        };

        super::encode(&header);
    }
}

pub fn encode(header: &Header, payload: &Payload) -> String {
    let header = general_purpose::STANDARD.encode(serde_json::to_string(header).unwrap());
    println!("{}", header);
    return "".to_owned();
}
