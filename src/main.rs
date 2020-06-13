use prometheus_exporter_base::render_prometheus;
use prometheus_jfplaybackreporting_exporter::{read_file, render};
use std::{env, path::Path};

#[tokio::main]
async fn main() {
    render_prometheus(
        ([0, 0, 0, 0], 32221).into(),
        {},
        |_request, _options| async move {
            let path_arg = env::args().into_iter().nth(1).unwrap();
            let path = Path::new(&path_arg);
            let reader = read_file(&path);
            Ok(render(reader))
        },
    )
    .await;
}
