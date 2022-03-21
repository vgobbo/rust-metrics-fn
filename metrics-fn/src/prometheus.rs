#[cfg(feature = "prometheus")]
pub fn record<T>(module: &str, result: &Result<(), T>, elapsed_s: f64)
where
	T: ToString,
{
	metrics_fn_prometheus::record(module, result, elapsed_s);
}

#[cfg(not(feature = "prometheus"))]
pub fn record<T>(_module: &str, _result: &Result<(), T>, _elapsed_s: f64)
where
	T: ToString,
{
	// do nothing.
}
