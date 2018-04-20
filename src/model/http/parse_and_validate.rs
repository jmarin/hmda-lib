#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyTs {
    ts: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyLar {
    lar: String,
}
