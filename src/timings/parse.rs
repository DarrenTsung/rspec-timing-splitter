use crate::timings::FileTiming;
use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref KV_RE: Regex = Regex::new(r##"\s*(\w+)="([^"]+)"\s*"##).unwrap();
}

pub fn parse_rspec_output(
    rspec_output: impl AsRef<str>,
) -> Result<Vec<FileTiming>, failure::Error> {
    let mut file_paths_to_total_times = HashMap::new();
    for line in rspec_output.as_ref().lines() {
        let mut file_path = None;
        let mut time = None;
        for caps in KV_RE.captures_iter(line) {
            let key = &caps[1];
            let value = &caps[2];

            match key {
                "file" => file_path = Some(value.to_string()),
                "time" => time = Some(value.parse::<f64>()?),
                _ => (),
            }
        }

        if let (Some(file_path), Some(time)) = (file_path, time) {
            *file_paths_to_total_times.entry(file_path).or_insert(0.0) += time
        }
    }

    Ok(file_paths_to_total_times
        .into_iter()
        .map(|(file_path, total_time)| FileTiming {
            file_path,
            total_time,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file_timings = parse_rspec_output(r####"
            <garbage>
            q-rj9r1-924i-ef0iw-fi-0iqp2ojlkj
            <testcase classname="spec.lib.deliveries.worker_spec" name="Some spec name" file="./spec/lib/deliveries/worker_spec.rb" time="0.164580"></testcase>
            <testcase classname="spec.lib.deliveries.worker_spec" name="Some spec name 2" file="./spec/lib/deliveries/worker_spec.rb" time="0.42"></testcase>
        "####).expect("no errors");

        assert_eq!(
            file_timings,
            vec![FileTiming {
                file_path: "./spec/lib/deliveries/worker_spec.rb".to_string(),
                total_time: 0.584580
            }]
        )
    }
}
