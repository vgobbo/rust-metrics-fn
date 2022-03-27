#[cfg(feature = "record-log")]
pub fn record(module: &str, fn_name: &str, result: &Result<(), ()>, elapsed_s: f64) {
	let result_text = match result {
		Ok(_) => "Ok".to_owned(),
		Err(_) => "Err".to_owned(),
	};

	log::info!("{} {} {} {}", module, fn_name, result_text, elapsed_s);
}

#[cfg(not(feature = "record-log"))]
pub fn record(_module: &str, _fn_name: &str, _result: &Result<(), ()>, _elapsed_s: f64) {
	// do nothing.
}
