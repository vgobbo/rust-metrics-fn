use std::thread::sleep;
use std::time::Duration;

use metrics_fn_codegen::measure;

#[test]
fn simple() {
	#[measure]
	fn some_method(a: u64, mut _b: u64, mut _c: &u64) {
		sleep(Duration::from_secs(a));
	}

	some_method(1, 2, &3);
}

pub async fn some() {
	todo!()
}
