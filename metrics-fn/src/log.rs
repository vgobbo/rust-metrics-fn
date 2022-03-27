#[cfg(feature = "record-log")]
pub fn record(module: &str, fn_name: &str, result: &Result<(), ()>, elapsed_s: f64) {
	match result {
		Ok(_) => log::info!("{} {} {}", module, fn_name, elapsed_s),
		Err(_) => log::error!("{} {} {}", module, fn_name, elapsed_s),
	}
}

#[cfg(not(feature = "record-log"))]
pub fn record(_module: &str, _fn_name: &str, _result: &Result<(), ()>, _elapsed_s: f64) {
	// do nothing.
}
