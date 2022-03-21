use std::time::Duration;

use metrics_fn::measure;
use simple_logger::SimpleLogger;

#[measure]
fn slow_method() {
	std::thread::sleep(Duration::from_secs(1));
}

fn main() {
	SimpleLogger::new().init().unwrap();

	slow_method();
}
