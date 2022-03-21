mod log;
mod prometheus;

pub use metrics_fn_codegen::measure;

pub fn record<T>(module: &str, result: Result<(), T>, elapsed_s: f64)
where
	T: ToString,
{
	log::record(module, &result, elapsed_s);
	prometheus::record(module, &result, elapsed_s);
}
