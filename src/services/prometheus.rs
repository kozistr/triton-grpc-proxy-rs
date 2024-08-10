use metrics_exporter_prometheus::{BuildError, Matcher, PrometheusBuilder};

pub(crate) fn prometheus_builer() -> Result<PrometheusBuilder, BuildError> {
    let duration_matcher: Matcher = Matcher::Suffix(String::from("duration"));
    let num_duration_buckets: usize = 35;
    let mut duration_buckets: Vec<f64> = Vec::with_capacity(num_duration_buckets);

    let mut value: f64 = 0.00001;
    for _ in 0..num_duration_buckets {
        value *= 1.5;
        duration_buckets.push(value);
    }

    let batch_size_matcher: Matcher = Matcher::Full(String::from("tgp_batch_size"));
    let batch_size_buckets: Vec<f64> = (0..13).map(|x: i32| 2.0_f64.powi(x)).collect();

    PrometheusBuilder::new()
        .set_buckets_for_metric(duration_matcher, &duration_buckets)?
        .set_buckets_for_metric(batch_size_matcher, &batch_size_buckets)
}
