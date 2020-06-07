use prometheus_exporter_base::{render_prometheus, MetricType, PrometheusMetric};
use prometheus_jfplaybackreporting_exporter::{read_file, render, sample};
use tokio::prelude::*;
#[tokio::main]
async fn main() {
    render_prometheus(
        ([0, 0, 0, 0], 32221).into(),
        {},
        |_request, _options| async move { Ok(sample()) },
    )
    .await;
}
