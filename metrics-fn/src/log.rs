#[cfg(feature = "log")]
pub fn record<T>(module: &str, fn_name: &str, result: &Result<(), T>, elapsed_s: f64)
where
	T: ToString,
{
	metrics_fn_log::record(module, fn_name, result, elapsed_s);
}

#[cfg(not(feature = "log"))]
pub fn record<T>(_module: &str, _fn_name: &str, _result: &Result<(), T>, _elapsed_s: f64)
where
	T: ToString,
{
	// do nothing.
}
