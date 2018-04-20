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

pub fn hmda_api_status(url: &str) -> Result<ServiceStatus, String> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let num_cpus = num_cpus::get();

    let client = Client::configure()
        .connector(HttpsConnector::new(num_cpus, &handle).unwrap())
        .build(&handle);

    let mut s = String::new();
    let uri = url.parse().unwrap();

    {
        let work = client.get(uri).and_then(|res| {
            res.body().for_each(|chunk| {
                s.push_str(str::from_utf8(&*chunk).unwrap());
                futures::future::ok(())
            })
        });
        core.run(work).unwrap();
    }

    let status: ServiceStatus = serde_json::from_str(&s).unwrap();
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmda_api_status() {
        let host = "https://ffiec-api.cfpb.gov/public/";
        let expected_status = ServiceStatus {
            status: String::from("OK"),
            service: String::from("service"),
            time: String::from("time"),
            host: String::from("host"),
        };
        let status = hmda_api_status(&host).unwrap();
        println!("{:?}", status);
        assert_eq!("OK", status.status);
        //assert_eq!(hmda_api_status(host).unwrap(), expected_status);
    }
}
