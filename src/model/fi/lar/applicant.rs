use model::fi::lar::ethnicity::Ethnicity;
use model::fi::lar::sex::Sex;
use model::fi::lar::race::Race;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Applicant {
    pub ethnicity: Ethnicity,
    pub race: Race,
    pub sex: Sex,
    pub age: u8,
    pub credit_score: i16,
    pub credit_score_type: i8,
    pub other_credit_score_model: String,
}

impl Applicant {
    pub fn sample_applicant() -> Applicant {
        Applicant {
            ethnicity: Ethnicity::sample_ethnicity(),
            race: Race::sample_race(),
            sex: Sex::sample_sex(),
            age: 30,
            credit_score: 750,
            credit_score_type: 1,
            other_credit_score_model: String::from("Other credit score model"),
        }
    }
}
