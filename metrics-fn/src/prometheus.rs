#[cfg(feature = "record-prometheus")]
pub fn record(module: &str, fn_name: &str, result: &Result<(), ()>, elapsed_s: f64) {
	internal::record(module, fn_name, result, elapsed_s)
}

#[cfg(not(feature = "record-prometheus"))]
pub fn record(_module: &str, _fn_name: &str, _result: &Result<(), ()>, _elapsed_s: f64) {
	// do nothing.
}

#[cfg(feature = "record-prometheus")]
mod internal {
	use lazy_static::lazy_static;
	use prometheus::{HistogramVec, register_histogram_vec};

	lazy_static! {
		static ref APPLICATION_METHOD_TIMINGS: HistogramVec = register_histogram_vec!(
			"application_method_timings",
			"Method execution timings in second.",
			&["mod", "fn", "res"],
			vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,]
		)
		.unwrap();
	}

	pub fn record(module: &str, fn_name: &str, result: &Result<(), ()>, elapsed_s: f64) {
		let result_text = match result {
			Ok(_) => "Ok".to_owned(),
			Err(_) => "Err".to_owned(),
		};

		let labels = &[module, fn_name, result_text.as_str()];
		APPLICATION_METHOD_TIMINGS.with_label_values(labels).observe(elapsed_s);
	}
}
