mod log;
mod prometheus;

pub use metrics_fn_codegen::measure;

pub fn record<T>(module: &str, fn_name: &str, result: Result<(), T>, elapsed_s: f64)
where
	T: ToString,
{
	log::record(module, fn_name, &result, elapsed_s);
	prometheus::record(module, fn_name, &result, elapsed_s);
}
