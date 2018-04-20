use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceStatus {
    pub status: String,
    pub service: String,
    pub time: String,
    pub host: String,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.status, self.service, self.time, self.host
        )
    }
}
