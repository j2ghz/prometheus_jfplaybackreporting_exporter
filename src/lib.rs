use csv::Error;
use prometheus_exporter_base::{render_prometheus, MetricType, PrometheusMetric};
use std::path::Path;

fn parse<T: std::str::FromStr>(
    string: &str,
) -> std::result::Result<T, <T as std::str::FromStr>::Err> {
    let parsed = string.parse::<T>();
    assert!(parsed.is_ok(), "Could not parse {}", string);
    parsed
}

pub fn read_file(path: &Path) -> csv::StringRecordsIntoIter<std::fs::File> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(path)
        .unwrap()
        .into_records()
}

pub fn render(records: csv::StringRecordsIntoIter<std::fs::File>) -> std::string::String {
    let pc = PrometheusMetric::new("watch_time", MetricType::Counter, "Watch time in seconds");
    let records = records.filter_map(|r| match r {
        Ok(record) => Some(record),
        Err(e) => None,
    });
    let mut s = pc.render_header();
    for r in records {
        s.push_str(
            &pc.render_sample(
                Some(&[
                    ("timestamp", r.get(0).unwrap()),
                    ("id1", r.get(1).unwrap()),
                    ("id2", r.get(2).unwrap()),
                    ("mediaType", r.get(3).unwrap()),
                    ("name", r.get(4).unwrap()),
                    ("playbackMethod", r.get(5).unwrap()),
                    ("app", r.get(6).unwrap()),
                    ("device", r.get(7).unwrap()),
                ]),
                r.get(8)
                    .unwrap_or_else(|| panic!("{:?}", r))
                    .parse::<u32>()
                    .unwrap(),
            ),
        );
    }
    s
}

pub fn sample() -> std::string::String {
    let reader = read_file(Path::new(
        "samples/PlaybackReportingBackup-20200607-021556.tsv",
    ));
    render(reader)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn complete() {
        let reader = read_file(Path::new(
            "samples/PlaybackReportingBackup-20200607-021556.tsv",
        ));
        let result = render(reader);
        assert_eq!("", result);
    }
}
