use log::info;

pub fn record<T>(module: &str, fn_name: &str, result: &Result<(), T>, elapsed_s: f64)
where
	T: ToString,
{
	let result_text = match result {
		Ok(_) => "Ok".to_owned(),
		Err(value) => value.to_string(),
	};

	info!("{} {} {} {}", module, fn_name, result_text, elapsed_s);
}
