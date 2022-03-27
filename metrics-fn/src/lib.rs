mod log;
mod prometheus;

pub use metrics_fn_codegen::measure;

pub fn record(module: &str, fn_name: &str, result: Result<(), ()>, elapsed_s: f64) {
	log::record(module, fn_name, &result, elapsed_s);
	prometheus::record(module, fn_name, &result, elapsed_s);
}
