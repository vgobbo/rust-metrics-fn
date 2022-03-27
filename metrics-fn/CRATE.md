# Function Metrics Runtime

This project is divided in a _build time_ module (`metrics-fn-codegen`), and a _runtime_ module (this module).

Simply including this module as a dependency and configuring the features for the desired metric recorders is enough to start using it.

For example, to enable collecting Prometheus metrics, use the following:
```toml
metrics-fn = { version = "0.1", features = [ "record-prometheus" ] }
```

## Recorders

The metrics are collected and sent to _recorders_, which can integrate with metrics systems like Prometheus.

In order to keep dependencies to a minimum, for each recorder there is an accompanying feature. These features are composed by `record-` and metrics system it enables.  **By default, all recorders are disabled**.

The following metrics systems are currently supported:
- Log (`record-log`): simply log the collected metrics, using `log::info!`; not really a metric collection system.
- Prometheus (`record-prometheus`): collect metrics to the default Prometheus metric registry, using the [Prometheus crate](https://crates.io/crates/prometheus).

### Prometheus Recorder

The Prometheus recorder collects metrics about the functions annotated with `#[measure]` to the `application_method_timings` histogram metric.

This metric has the following labels:

- `mod`: module name.
- `fn`: function name.
- `res`: result (`Ok` or `Err` if the annotated function result is named `Result`; always `Ok` otherwise).

The following buckets are used: 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0.

In order to expose the collected metrics, a web application framework like [Rocket](https://rocket.rs) or [Actix Web](https://actix.rs/) must be manually included and configured. 

See [metrics-fn-example-prometheus](../metrics-fn-example-prometheus) for an example.

## To-Do List

The following are planned features, by priority:

- **[MEDIUM]** Record type name associated with `impl`.
- **[LOW]** Configurable Prometheus metrics names and buckets.
- **[DONE]** Record `core::result::Result` successes/errors.
