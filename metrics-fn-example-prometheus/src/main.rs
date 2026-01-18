mod delay;
mod fortunes;
mod fortunes_repository;
mod hello;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate metrics_fn;

use prometheus::TextEncoder;

#[measure]
#[get("/metrics")]
async fn metrics() -> String {
	let metric_families = prometheus::gather();
	let encoder = TextEncoder::new();

	encoder.encode_to_string(&metric_families).unwrap()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	rocket::build()
		.mount("/", routes![metrics])
		.mount("/hello", hello::routes())
		.mount("/delay", delay::routes())
		.mount("/fortunes", fortunes::routes())
		.ignite()
		.await?
		.launch()
		.await
		.map(drop)
}
