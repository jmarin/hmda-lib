use std::fmt;
use model::fi::ts::contact::Contact;
use model::fi::ts::address::Address;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransmittalSheet {
    pub id: u8,
    pub institution_name: String,
    pub year: u16,
    pub quarter: u8,
    pub contact: Contact,
    pub agency: u8,
    pub total_lines: i32,
    pub tax_id: String,
    #[serde(rename = "LEI")]
    pub lei: String,
}

impl TransmittalSheet {
    pub fn ts_sample() -> TransmittalSheet {
        TransmittalSheet {
            id: 1,
            institution_name: String::from("Bank 0"),
            year: 2018,
            quarter: 4,
            contact: Contact {
                name: String::from("Jane Smith"),
                phone: String::from("111-111-1111"),
                email: String::from("jane.smith@bank0.com"),
                address: Address {
                    street: String::from("123 Main St"),
                    city: String::from("Washington"),
                    state: String::from("DC"),
                    zip_code: String::from("20001"),
                },
            },
            agency: 9,
            total_lines: 1000,
            tax_id: String::from("99-999999"),
            lei: String::from("10Bx939c5543TqA1144M"),
        }
    }
}

impl fmt::Display for TransmittalSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.institution_name,
            self.year,
            self.quarter,
            self.contact,
            self.agency,
            self.total_lines,
            self.tax_id,
            self.lei
        )
    }
}

#[cfg(test)]
mod tests {

    extern crate serde;
    extern crate serde_json;
    use super::*;

    #[test]
    fn test_ts_sample() {
        let address = Address {
            street: String::from("123 Main St"),
            city: String::from("Washington"),
            state: String::from("DC"),
            zip_code: String::from("20001"),
        };
        assert_eq!(address.street, "123 Main St");
        assert_eq!(address.city, "Washington");
        assert_eq!(address.state, "DC");
        assert_eq!(address.zip_code, "20001");
        let ts = TransmittalSheet::ts_sample();
        assert_eq!(ts.id, 1);
        assert_eq!(ts.to_string(), "1|Bank 0|2018|4|Jane Smith|111-111-1111|jane.smith@bank0.com|123 Main St|Washington|DC|20001|9|1000|99-999999|10Bx939c5543TqA1144M");
        assert_eq!(ts.agency, 9);
        assert_eq!(ts.contact.name, "Jane Smith");
    }

    #[test]
    fn test_transmittal_sheet_serialize() {
        let ts = TransmittalSheet::ts_sample();
        let json = serde_json::to_string(&ts).unwrap();
        let deserialized = serde_json::from_str(&json).unwrap();
        assert_eq!(ts, deserialized);
    }

}
