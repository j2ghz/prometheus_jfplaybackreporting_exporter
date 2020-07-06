use chrono::prelude::*;
use prometheus_exporter_base::{MetricType, PrometheusMetric};
use std::path::Path;

pub fn read_file(path: &Path) -> csv::StringRecordsIntoIter<std::fs::File> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(path)
        .unwrap()
        .into_records()
}

pub fn render(records: csv::StringRecordsIntoIter<std::fs::File>) -> std::string::String {
    let pc = PrometheusMetric::new(
        "jellyfin_watchtime",
        MetricType::Counter,
        "Watch time in seconds",
    );
    let records = records.filter_map(|r| match r {
        Ok(record) => Some(record),
        Err(e) => {
            eprintln!("Invalid record: {}", e);
            None
        }
    });
    let mut s = pc.render_header();
    for r in records {
        let timestamp_str = r.get(0).unwrap();
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%F %T%.f").unwrap();
        let timestamp_with_tz = Local::now()
            .offset()
            .from_local_datetime(&timestamp)
            .unwrap();
        s.push_str(
            &pc.render_sample(
                Some(&[
                    ("timestamp", r.get(0).unwrap()),
                    ("userId", r.get(1).unwrap()),
                    ("itemId", r.get(2).unwrap()),
                    ("itemType", r.get(3).unwrap()),
                    ("itemName", r.get(4).unwrap()),
                    ("playbackMethod", r.get(5).unwrap()),
                    ("clientName", r.get(6).unwrap()),
                    ("deviceName", r.get(7).unwrap()),
                ]),
                r.get(8)
                    .unwrap_or_else(|| panic!("{:?}", r))
                    .parse::<u32>()
                    .unwrap(),
                Some(timestamp_with_tz.timestamp_millis()),
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
    use pretty_assertions::assert_ne;

    #[test]
    #[ignore = "Local file required"]
    fn complete() {
        let reader = read_file(Path::new(
            "samples/PlaybackReportingBackup-20200607-021556.tsv",
        ));
        let result = render(reader);
        assert_ne!("", result);
    }
}
