# Function Metrics

Collects execution metrics for the annotated function.

The following metrics are collected:
- Execution time.
- Success (if function returns a `core::result::Result`-like type.

The metrics are sent to _recorders_, which can be thought as backends, which can send for metric aggregation services, for example.

Currently, there are the following recorders implemented:
- Log: simply log the metrics.
- Prometheus: collect the metrics so they can be sent to Prometheus.