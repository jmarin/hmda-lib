use std::fmt;
use model::fi::ts::address::Address;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: Address,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.name, self.phone, self.email, self.address
        )
    }
}
