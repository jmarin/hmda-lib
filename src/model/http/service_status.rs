use std::fmt;
use std::cmp::PartialEq;

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

impl PartialEq for ServiceStatus {
    fn eq(&self, other: &ServiceStatus) -> bool {
        self.status == other.status && self.service == other.service && self.time == other.time
            && self.host == other.host
    }
}

#[cfg(test)]
mod tests {

    extern crate serde;
    extern crate serde_json;
    use super::*;

    #[test]
    fn test_service_status_serialize() {
        let status = ServiceStatus {
            status: String::from("status"),
            service: String::from("service"),
            time: String::from("time"),
            host: String::from("host"),
        };
        let json = serde_json::to_string(&status).unwrap();
        let deserialized = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

}
