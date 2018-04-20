use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.street, self.city, self.state, self.zip_code
        )
    }
}
