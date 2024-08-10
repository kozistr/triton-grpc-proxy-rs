use metrics_exporter_prometheus::PrometheusHandle;
use ntex::web::types::State;
use ntex::web::{get, ServiceConfig};

#[utoipa::path(
    get,
    path = "/metrics",
    responses(
      (status = 200, description = "Get prometheus metrics", body = String),
    ),
  )]
#[get("/metrics")]
pub async fn get_metrics(prom_handle: State<PrometheusHandle>) -> String {
    prom_handle.render()
}

pub fn ntex_config(cfg: &mut ServiceConfig) {
    cfg.service(get_metrics);
}
