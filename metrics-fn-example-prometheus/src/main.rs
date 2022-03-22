#[macro_use]
extern crate rocket;
#[macro_use]
extern crate metrics_fn;

use std::str::FromStr;

use prometheus::{TextEncoder, TEXT_FORMAT};
use rocket::http::{ContentType, MediaType, Status};
use rocket::response::content::Custom;
use rocket::tokio::time::{sleep, Duration};

#[measure]
#[get("/")]
async fn hello_world() -> &'static str {
	"Hello, world!"
}

#[measure]
#[get("/hello/<name>")]
async fn hello_name(name: String) -> String {
	format!("Hello, {}!", name)
}

#[measure]
#[get("/delay")]
async fn delay_random_ms() -> String {
	// no more than 20 seconds.
	let ms = (rand::random::<u32>() % 20_000) as u64;
	sleep(Duration::from_millis(ms)).await;

	format!("Waited for {}ms", ms)
}

#[measure]
#[get("/delay/<ms>")]
async fn delay_ms(ms: u64) -> String {
	sleep(Duration::from_millis(ms)).await;
	format!("Waited for {}ms", ms)
}

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
		.mount(
			"/",
			routes![hello_world, hello_name, delay_random_ms, delay_ms, metrics],
		)
		.ignite()
		.await?
		.launch()
		.await
}
