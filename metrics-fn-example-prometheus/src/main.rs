mod delay;
mod fortunes;
mod fortunes_repository;
mod hello;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate metrics_fn;

use std::str::FromStr;

use prometheus::{TextEncoder, TEXT_FORMAT};
use rocket::http::{ContentType, MediaType, Status};
use rocket::response::content::Custom;

#[measure]
#[get("/metrics")]
async fn metrics() -> Result<Custom<String>, Status> {
	let metric_families = prometheus::gather();
	let encoder = TextEncoder::new();
	match encoder.encode_to_string(&metric_families) {
		Ok(data) => Ok(Custom(ContentType(MediaType::from_str(TEXT_FORMAT).unwrap()), data)),
		Err(_) => Err(Status::InternalServerError),
	}
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
}
