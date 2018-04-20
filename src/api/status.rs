extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate num_cpus;
extern crate serde_json;
extern crate tokio_core;

use std::str;
use self::tokio_core::reactor::Core;
use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::futures::future::Future;
use self::futures::Stream;
use model::http::service_status::ServiceStatus;
use model::http::util::get_json;

pub fn hmda_api_status(url: &String) -> Result<ServiceStatus, String> {
    let s = get_json(url);
    let status: ServiceStatus = serde_json::from_str(&s).unwrap();
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmda_api_status() {
        let host = String::from("https://ffiec-api.cfpb.gov/public/");
        let expected_status = ServiceStatus {
            status: String::from("OK"),
            service: String::from("service"),
            time: String::from("time"),
            host: String::from("host"),
        };
        let status = hmda_api_status(&host).unwrap();
        println!("{:?}", status);
        assert_eq!("OK", status.status);
        assert_eq!("hmda-public-api", status.service);
    }
}