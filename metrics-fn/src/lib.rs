//! # Function Metrics
//!
//! Collects execution metrics for the annotated function.
//!
//! The following metrics are collected:
//! - Execution time.
//! - Success (if function returns a `core::result::Result`-like type.
//!
//! ## Usage
//!
//! Include the dependency and enable the desired recorders.
//!
//! For example, the Prometheus recorder can be enabled like the following:
//!
//! ```toml
//! metrics-fn = { version = "0.1", features = [ "record-prometheus" ] }
//! ```
//!
//! After that, just annotate the desired functions with `#[measure]`.
//!
//! ```
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! use metrics_fn_codegen::measure;
//!
//! #[measure]
//! pub fn super_slow_function(a: u64, b: u64) {
//! 	sleep(Duration::from_millis(a * b));
//! }
//! ```
//!
//! Notice that there is a performance impact due to the metric collection on **every** call to the function. Thus it is
//! advised to use the annotation on functions where the performance impact is negligible, like on function that makes
//! calls to external services or database queries.
//!
//! ## Recorders
//!
//! The metrics are collected and sent to _recorders_, which can integrate with metrics systems like Prometheus.
//!
//! In order to keep dependencies to a minimum, for each recorder there is an accompanying feature. These features are
//! composed by `record-` and metrics system it enables.  **By default, all recorders are disabled**.
//!
//! The following metrics systems are currently supported:
//! - Log (`record-log`): simply log the collected metrics, using `log::info!`; not really a metric collection system.
//! - Prometheus (`record-prometheus`): collect metrics to the default Prometheus metric registry, using the [Prometheus
//!   crate](https://crates.io/crates/prometheus).
//!
//! ### Log Recorder (`record-log`)
//!
//! The log recorder is not really useful: it was created to help with development.
//!
//! It simply logs the metrics using `log::info!` or `log::error!`.
//!
//! ### Prometheus Recorder (`record-prometheus`)
//!
//! The Prometheus recorder collects metrics about the functions annotated with `#[measure]` to the
//! `application_method_timings` histogram metric.
//!
//! This metric has the following labels:
//!
//! - `mod`: module name.
//! - `fn`: function name.
//! - `res`: result (`Ok` or `Err` if the annotated function result is named `Result`; always `Ok` otherwise).
//!
//! The following buckets are used: 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0.
//!
//! In order to expose the collected metrics, a web application framework like [Rocket](https://rocket.rs) or [Actix Web](https://actix.rs/) must be manually included and configured.
//!
//! See [metrics-fn-example-prometheus](../metrics-fn-example-prometheus) for an example.
//!
//! ## Versioning
//!
//! [Semantic Versioning](https://semver.org/) is used. Changes to the collected metrics are also considered in the versioning.
//!
//! ## To-Do List
//!
//! The following are planned features, by priority:
//!
//! **This list is not a commitment: items might be added or removed of the list.**
//!
//! - **[MEDIUM]** Record type name associated with `impl`.
//! - **[LOW]** Configurable Prometheus metrics names and buckets.

mod log;
mod prometheus;

pub use metrics_fn_codegen::measure;

pub fn record(module: &str, fn_name: &str, result: Result<(), ()>, elapsed_s: f64) {
	log::record(module, fn_name, &result, elapsed_s);
	prometheus::record(module, fn_name, &result, elapsed_s);
}
