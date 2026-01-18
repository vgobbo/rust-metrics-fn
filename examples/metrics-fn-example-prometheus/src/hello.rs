use rocket::Route;

#[measure]
#[get("/")]
pub async fn world() -> &'static str {
	"Hello, world!"
}

#[measure]
#[get("/<name>")]
pub async fn name(name: String) -> String {
	format!("Hello, {}!", name)
}

pub fn routes() -> Vec<Route> {
	routes![world, name]
}
