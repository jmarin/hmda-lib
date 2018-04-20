extern crate serde;
extern crate serde_json;

use model::http::parse_and_validate::VerifyTs;
use model::fi::ts::transmittal_sheet::TransmittalSheet;
use api::util::{get_url, post_json};

pub fn ts_parse(verify_ts: &VerifyTs) -> TransmittalSheet {
    let root_url = get_url();
    let url = root_url + "ts/parse";
    let json = serde_json::to_string(&verify_ts).unwrap();
    let ts = post_json(&url, json);
    let deserialized: TransmittalSheet = serde_json::from_str(&ts).unwrap();
    deserialized
}

#[cfg(test)]
mod tests {
    use super::*;

    use api::util::get_url;
    use model::fi::ts::transmittal_sheet::TransmittalSheet;
    use model::http::parse_and_validate::VerifyTs;

    #[test]
    fn test_ts_parse() {
        let ts = TransmittalSheet::ts_sample();
        let verify_ts = VerifyTs { ts: ts.to_string() };
        let response = ts_parse(&verify_ts);
        println!("{}", response);
        assert_eq!(response, ts);
    }

}
