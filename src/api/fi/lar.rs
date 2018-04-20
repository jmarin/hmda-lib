extern crate serde;
extern crate serde_json;

use model::fi::lar::loan_application_register::LoanApplicationRegister;
use model::http::parse_and_validate::VerifyLar;
use api::util::{get_url, post_json};

pub fn lar_parse(verify_lar: &VerifyLar) -> LoanApplicationRegister {
    let root_url = get_url();
    let url = root_url + "lar/parse";
    let json = serde_json::to_string(&verify_lar).unwrap();
    let lar = post_json(&url, json);
    let deserialized: LoanApplicationRegister = serde_json::from_str(&lar).unwrap();
    deserialized
}

#[cfg(test)]
mod tests {
    use super::*;

    use model::fi::lar::loan_application_register::LoanApplicationRegister;
    use model::http::parse_and_validate::VerifyLar;

    #[test]
    fn test_lar_parse() {
        let lar = LoanApplicationRegister::lar_sample();
        let verify_lar = VerifyLar {
            lar: lar.to_string(),
        };
        let response = lar_parse(&verify_lar);
        println!("{}", response);
        println!("{}", lar);
        assert_eq!(response, lar);
    }
}
