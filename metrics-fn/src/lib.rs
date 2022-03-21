pub use metrics_fn_codegen::measure;

#[cfg(feature = "log")]
pub fn record<T>(module: &str, result: Result<(), T>, elapsed_ns: f64)
where
	T: ToString,
{
	metrics_fn_log::record(module, result, elapsed_ns);
}
