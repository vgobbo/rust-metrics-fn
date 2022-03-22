#[macro_use]
extern crate rocket;
#[macro_use]
extern crate metrics_fn;

use rocket::tokio::time::{sleep, Duration};

#[get("/")]
async fn hello_world() -> &'static str {
	"Hello, world!"
}

#[get("/hello/<name>")]
async fn hello_name(name: String) -> String {
	format!("Hello, {}!", name)
}

#[get("/delay")]
async fn delay_random_ms() -> String {
	// no more than 20 seconds.
	let seconds = (rand::random::<u32>() % 20_000) as u64;
	sleep(Duration::from_millis(seconds)).await;

	format!("Waited for {} seconds", seconds)
}

#[get("/delay/<ms>")]
async fn delay_ms(ms: u64) -> String {
	sleep(Duration::from_millis(ms)).await;
	format!("Waited for {}ms", ms)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	rocket::build()
		.mount("/", routes![hello_world, hello_name, delay_random_ms, delay_ms])
		.ignite()
		.await?
		.launch()
		.await
}
