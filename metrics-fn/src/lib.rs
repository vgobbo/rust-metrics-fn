pub use metrics_fn_codegen::measure;

pub fn record<T>(module: &str, result: Result<(), T>, elapsed_ns: f64)
where
	T: ToString,
{
	if cfg!(feature = "log") {
		metrics_fn_log::record(module, result, elapsed_ns);
	}
}
