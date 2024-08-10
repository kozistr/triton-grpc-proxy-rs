use metrics_exporter_prometheus::PrometheusHandle;
use ntex::web::get;
use ntex::web::types::State;

#[get("/metrics")]
pub async fn get_metrics(prom_handle: State<PrometheusHandle>) -> String {
    prom_handle.render()
}
