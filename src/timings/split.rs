use crate::timings::FileTiming;

/// Splits timings into N (where N is total_splits) buckets, attempting to balance
/// the buckets as much as possible.
///
/// This is a deterministic algorithm and must always produce the same result for multiple runs.
pub fn split_timings(timings: &[FileTiming], total_splits: u32) -> Vec<Vec<FileTiming>> {
    if total_splits == 0 {
        return vec![];
    }

    let mut timings = timings.iter().cloned().collect::<Vec<_>>();
    // descending order
    timings.sort_by(|a, b| b.total_time.partial_cmp(&a.total_time).unwrap());

    struct TimingAggregator {
        timings: Vec<FileTiming>,
        total_time: f64,
    }

    let mut buckets = vec![];
    for _ in 0..total_splits {
        buckets.push(TimingAggregator {
            timings: vec![],
            total_time: 0.0,
        });
    }

    // from largest timing to smallest
    for timing in timings {
        let mut min_bucket_index = 0;
        for (index, bucket) in buckets.iter().enumerate().skip(1) {
            if bucket.total_time < buckets[min_bucket_index].total_time {
                min_bucket_index = index;
            }
        }

        buckets[min_bucket_index].total_time += timing.total_time;
        buckets[min_bucket_index].timings.push(timing);
    }

    buckets
        .into_iter()
        .map(|aggregate| aggregate.timings)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ft(file_path: impl Into<String>, total_time: f64) -> FileTiming {
        FileTiming {
            file_path: file_path.into(),
            total_time,
        }
    }

    fn check_produces_the_same_result_over_multiple_runs_and_ret(
        timings: Vec<FileTiming>,
        total_splits: u32,
    ) -> Vec<Vec<FileTiming>> {
        let first_result = split_timings(timings.clone(), total_splits);
        for _ in 0..10 {
            assert_eq!(first_result, split_timings(timings.clone(), total_splits));
        }
        first_result
    }

    #[test]
    fn less_timings_than_buckets() {
        let buckets = check_produces_the_same_result_over_multiple_runs_and_ret(
            vec![ft("a", 10.0), ft("b", 20.0)],
            3,
        );
        assert_eq!(
            buckets,
            vec![vec![ft("b", 20.0)], vec![ft("a", 10.0)], vec![]]
        )
    }

    #[test]
    fn more_timings_than_buckets() {
        let buckets = check_produces_the_same_result_over_multiple_runs_and_ret(
            vec![ft("a", 10.0), ft("b", 20.0), ft("c", 15.0), ft("d", 5.0)],
            2,
        );
        assert_eq!(
            buckets,
            vec![
                vec![ft("b", 20.0), ft("d", 5.0)],
                vec![ft("c", 15.0), ft("a", 10.0)]
            ]
        )
    }
}
