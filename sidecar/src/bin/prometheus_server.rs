#[macro_use]
extern crate lazy_static;
#[macro_use]

extern crate prometheus;
extern crate hyper;
extern crate hyper_router;

use hyper::{header::CONTENT_TYPE, rt::Future, Body, Request, Response, Server};
use hyper_router::{Route, RouterBuilder, RouterService};
use prometheus::{Counter, Encoder, TextEncoder};
// use std::{thread, time};

lazy_static! {
    static ref APP_COUNTER: Counter = register_counter!(opts!(
        "rg_app_counter",
        "First example of a counter from Rust.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
}

fn metrics_handler(_: Request<Body>) -> Response<Body> {
    println!("Received request for metrics");
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .expect("Failed to construct response");

    response
}

fn counter_handler(_: Request<Body>) -> Response<Body> {
    println!("Received request for counter");
    APP_COUNTER.inc();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/plain".to_string())
        .body(Body::from("done"))
        .expect("Failed to construct response");

    response
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/inc").using(counter_handler))
        .add(Route::get("/metrics").using(metrics_handler))
        .build();

    Ok(RouterService::new(router))
}

fn main() {
    let addr = ([127, 0, 0, 1], 4000).into();
    println!("HTTP ADDRESS: {:?}", addr);

    let server = Server::bind(&addr).serve(router_service).map_err(|e| {
        eprintln!("Server error: {}", e);
    });

    hyper::rt::run(server);

    // let counter_opts = Opts::new("test_counter", "test_counter help");
    // let counter = Counter::with_opts(counter_opts).unwrap();

    let r = prometheus::default_registry();
    r.register(Box::new(APP_COUNTER.clone())).unwrap();

    // let _ = thread::spawn(move || {
    //     for _ in 0..100 {
    //         let five_secs = time::Duration::from_millis(5000);
    //         thread::sleep(five_secs);
    //         println!("Increment counter");
    //         APP_COUNTER.inc();
    //     }
    // })
    // .join();

    ////////////////////
}
