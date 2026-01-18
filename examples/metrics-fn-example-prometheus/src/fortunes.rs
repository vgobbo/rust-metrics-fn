use rocket::Route;

#[measure]
#[get("/")]
pub fn fortunes() -> String {
	let repository = crate::fortunes_repository::FortunesRepository::instance();
	repository.random()
}

pub fn routes() -> Vec<Route> {
	routes![fortunes]
}
