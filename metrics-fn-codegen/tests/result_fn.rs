use metrics_fn_codegen::measure;

#[test]
pub fn std_result() {
	#[measure]
	fn some_method(a: u64, b: u64) -> core::result::Result<u64, u64> {
		if a > b {
			Ok(a)
		} else {
			Err(b)
		}
	}

	let _ = some_method(50, 2);
}

#[test]
pub fn result() {
	#[measure]
	fn some_method(a: u64, b: u64) -> Result<u64, u64> {
		if a > b {
			Ok(a)
		} else {
			Err(b)
		}
	}

	let _ = some_method(50, 2);
}

mod metrics_fn {
	pub fn record(_: &str, _: &str, _: Result<(), ()>, _: f64) {
		// do nothing.
	}
}
