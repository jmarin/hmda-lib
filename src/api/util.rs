extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate num_cpus;
extern crate tokio_core;

use std::str;
use std::env;
use self::tokio_core::reactor::Core;
use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::futures::future::Future;
use self::futures::Stream;

pub fn get_url() -> String {
    let url = env::var("HMDA_URL");

    match url {
        Ok(h) => h,
        Err(_) => {
            println!("HMDA_URL environment variable not found, using default instead");
            String::from("https://ffiec-api.cfpb.gov/public/")
        }
    }
}

pub fn get_json(url: &String) -> String {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let num_cpus = num_cpus::get();

    let client = Client::configure()
        .connector(HttpsConnector::new(num_cpus, &handle).unwrap())
        .build(&handle);

    let mut response = String::new();
    let uri = url.parse().unwrap();

    {
        let work = client.get(uri).and_then(|res| {
            res.body().for_each(|chunk| {
                response.push_str(str::from_utf8(&*chunk).unwrap());
                futures::future::ok(())
            })
        });
        core.run(work).unwrap();
    }

    response
}
