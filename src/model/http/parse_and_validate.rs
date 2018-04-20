#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyTs {
    pub ts: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyLar {
    pub lar: String,
}
