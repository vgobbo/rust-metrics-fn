use rocket::tokio::time::{sleep, Duration};
use rocket::Route;

#[measure]
#[get("/")]
pub async fn random() -> String {
	// no more than 20 seconds.
	let ms = (rand::random::<u32>() % 20_000) as u64;
	sleep(Duration::from_millis(ms)).await;

	format!("Waited for {}ms", ms)
}

#[measure]
#[get("/<ms>")]
pub async fn ms(ms: u64) -> String {
	sleep(Duration::from_millis(ms)).await;
	format!("Waited for {}ms", ms)
}

pub fn routes() -> Vec<Route> {
	routes![random, ms]
}
