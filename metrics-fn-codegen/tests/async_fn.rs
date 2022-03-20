use std::time::Duration;

use metrics_fn_codegen::measure;
use tokio::time::sleep;

#[tokio::test]
pub async fn many_args() {
	#[measure]
	async fn some_method(a: u64, mut _b: u64, mut _c: &u64) {
		sleep(Duration::from_millis(a)).await;
	}

	some_method(50, 2, &3).await;
}

#[tokio::test]
pub async fn one_args() {
	#[measure]
	async fn some_method(a: u64) {
		sleep(Duration::from_millis(a)).await;
	}

	some_method(50).await;
}

#[tokio::test]
pub async fn no_args() {
	#[measure]
	async fn some_method() {
		sleep(Duration::from_millis(50)).await;
	}

	some_method().await;
}
