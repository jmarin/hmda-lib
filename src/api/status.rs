extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate num_cpus;
extern crate serde_json;
extern crate tokio_core;

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
    use model::http::util::get_url;

    #[test]
    fn test_hmda_api_status() {
        //let host = String::from("https://ffiec-api.cfpb.gov/public/");
        let host = get_url();
        let status = hmda_api_status(&host).unwrap();
        println!("{:?}", status);
        assert_eq!("OK", status.status);
        assert_eq!("hmda-public-api", status.service);
    }
}
