use futures::stream::StreamExt;
use prometheus_exporter_base::render_prometheus;
use prometheus_jfplaybackreporting_exporter::{read_file, render};
use std::{env, path::Path};
use tokio::fs::read_dir;

#[tokio::main]
async fn main() {
    render_prometheus(
        ([0, 0, 0, 0], 32221).into(),
        {},
        |_request, _options| async move {
            let path_arg = env::args().nth(1).expect("Expected a single parameter with path to the directory with the exported files");
            let path = Path::new(&path_arg);
            let dir = read_dir(path).await.expect("Couldn't read directory");
            let (file, _) = dir
                .fold(None, |acc, it| async {
                    let file = it.expect("Couldn't read file info");
                    let meta = file.metadata().await.expect("Couldn't read file metadata");
                    let modif = meta.modified().expect("Couldn't read file's modified date");
                    if let Some((path, date)) = acc {
                        if date > modif {
                            Some((path, date))
                        } else {
                            Some((file.path(), modif))
                        }
                    } else {
                        Some((file.path(), modif))
                    }
                })
                .await
                .expect("No files found");
            eprintln!("Reading {}", file.display());
            let reader = read_file(&file);
            Ok(render(reader))
        },
    )
    .await;
}
