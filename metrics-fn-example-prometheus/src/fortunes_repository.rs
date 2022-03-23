use std::vec::Vec;

use lazy_static::lazy_static;

lazy_static! {
	static ref FORTUNES: Vec<String> = {
		vec![
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_owned(),
			"Aliquam porta mi ac nibh tincidunt sollicitudin.".to_owned(),
			"Sed posuere ante eu risus viverra tincidunt.".to_owned(),
			"Quisque consequat arcu ut ante porta scelerisque.".to_owned(),
			"Nulla facilisis nibh nec urna accumsan imperdiet.".to_owned(),
			"Duis scelerisque ex a sem congue, vel congue turpis blandit.".to_owned(),
			"Etiam eget odio vel nisl rhoncus dignissim id consequat nisi.".to_owned(),
			"Nunc convallis felis nec urna finibus fermentum.".to_owned(),
			"Suspendisse sodales nunc non cursus pharetra.".to_owned(),
			"Nam facilisis ipsum eu lacus egestas fermentum.".to_owned(),
			"Praesent sagittis lectus ac arcu dignissim, consequat bibendum nibh pretium.".to_owned(),
			"Mauris eu lectus venenatis, egestas arcu mollis, molestie arcu.".to_owned(),
		]
	};
}

pub struct FortunesRepository {}

// This is artificially complicate to show metrics being recorded for member functions.
impl FortunesRepository {
	pub fn instance() -> FortunesRepository {
		FortunesRepository::new()
	}

	fn new() -> FortunesRepository {
		FortunesRepository {}
	}

	#[measure]
	pub fn random(&self) -> String {
		let index = rand::random::<usize>() % FORTUNES.len();
		FORTUNES.get(index).unwrap().to_owned()
	}
}
