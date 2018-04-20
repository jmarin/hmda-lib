#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LarIdentifier {
    pub id: u8,
    #[serde(rename = "LEI")]
    pub lei: String,
    #[serde(rename = "NMLSRIdentifier")]
    pub nmls_identifier: String,
}

impl LarIdentifier {
    pub fn sample_lar_identifier() -> LarIdentifier {
        LarIdentifier {
            id: 2,
            lei: "10Bx939c5543TqA1144M".to_string(),
            nmls_identifier: "12345".to_string(),
        }
    }
}
