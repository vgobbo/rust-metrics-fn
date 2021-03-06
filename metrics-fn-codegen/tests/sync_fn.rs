use std::thread::sleep;
use std::time::Duration;

use metrics_fn_codegen::measure;

#[test]
pub fn many_args() {
	#[measure]
	fn some_method(a: u64, mut _b: u64, mut _c: &u64) {
		sleep(Duration::from_millis(a));
	}

	some_method(50, 2, &3);
}

#[test]
pub fn one_args() {
	#[measure]
	fn some_method(a: u64) {
		sleep(Duration::from_millis(a));
	}

	some_method(50);
}

#[test]
pub fn no_args() {
	#[measure]
	fn some_method() {}

	some_method();
}

#[test]
pub fn with_result() {
	#[measure]
	fn sum(a: u32, b: u32) -> u32 {
		a + b
	}

	assert_eq!(3, sum(1, 2));
}

mod metrics_fn {
	pub fn record(_: &str, _: &str, _: Result<(), ()>, _: f64) {
		// do nothing.
	}
}
