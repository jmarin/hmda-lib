#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub race1: i8,
    pub race2: i8,
    pub race3: i8,
    pub race4: i8,
    pub race5: i8,
    pub other_native_race: String,
    pub other_asian_race: String,
    pub other_pacific_islander_race: String,
    pub race_observed: u8,
}

impl Race {
    pub fn sample_race() -> Race {
        Race {
            race1: 5,
            race2: 7,
            race3: 7,
            race4: 7,
            race5: 7,
            other_native_race: "".to_string(),
            other_asian_race: "".to_string(),
            other_pacific_islander_race: "".to_string(),
            race_observed: 3,
        }
    }
}
