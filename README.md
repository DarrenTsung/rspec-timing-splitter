# rspec-timing-tool
Rust binary that parses rspecs timing information to split test files

# How to use
Assuming you've built the release binary with `cargo build --release` and are now in your Rails project:

```bash
# Run your tests, outputting the results into rspec-results.xml
bundle exec rspec --format RspecJunitFormatter -o rspec-results.xml

# Run the tool to parse the rspec format into rspec-parsed.txt
rspec-timing-tool parse rspec-results.xml --output rspec-parsed.txt

# Shows an overview of how the files will be split with 5 buckets
#
# Example output:
# [BUCKET 1 - 14.31s] app_spec:14.31s
# [BUCKET 2 - 12.21s] importer_spec:11.09s, app_white_list_spec:1.04s, platform_spec:0.08s
# [BUCKET 3 - 12.36s] segment_spec:9.71s, user_spec:1.44s, player_lookup_spec:0.90s, location_spec:0.31s
# [BUCKET 4 - 12.21s] filter_spec:6.95s, automatic_spec:3.31s, shard_spec:1.06s, utils_spec:0.47s, bee_free_controller_spec:0.33s, database_spec:0.08s
# [BUCKET 5 - 12.72s] organization_spec:6.18s, notice_spec:3.43s, player_spec:3.11s
rspec-timing-tool analyze --total-splits 5 rspec-parsed.txt

# Outputs the file paths of the specs that fall into the current-split specified
#
# Example output (for --current-split 0 which is BUCKET 1):
# ./spec/models/app_spec.rb
rspec-timing-tool split --current-split 0 --total-splits 5 rspec-parsed.txt

# Example output (for --current-split 3 which is BUCKET 4):
# ./spec/models/filter_spec.rb ./spec/workers/automatic_spec.rb ./spec/lib/shard_spec.rb ./spec/lib/one_signal/utils_spec.rb ./spec/controllers/bee_free_controller_spec.rb ./spec/lib/database_spec.rb
rspec-timing-tool split --current-split 3 --total-splits 5 rspec-parsed.txt
```

# CircleCI
This tool was built to replace CircleCI's built-in method of test splitting as it
was doing a very poor job of balancing the containers (and there was no way to reset
the cached timing data).

Here's the suggested way from the docs (https://circleci.com/docs/2.0/parallelism-faster-jobs/#running-split-tests):
```bash
TESTFILES=$(circleci tests glob "spec/**/*.rb" | circleci tests split --split-by=timings)
bundle exec rspec -- ${TESTFILES}
```

Here's an example of how to use it in your `.circleci/config.yml`:
```bash
TESTFILES=$(rspec-timing-tool split --total-splits $CIRCLE_NODE_TOTAL --current-split $CIRCLE_NODE_INDEX rspec-parsed.txt)
bundle exec rspec -- ${TESTFILES}
```

This assumes that you have the `rspec-timing-tool` built for your container in
the working directory and that your parsed timing data is in a file named
`rspec-parsed.txt` in the working directory.

# (Anecdotal) Results
Before this tool our test timings were skewed terribly and not balancing at all,
even though we push many changes per day. The latest build was split between 4 containers
that finished in 3:49, 5:15, 12:24, and 11:39 respectively.

After the changes, we had 4 containers finish in 8:10, 7:41, 8:20, and 9:29 respectively.
Obviously, it's not perfectly balanced as the tests can take different amounts of time
and we're balancing off of a single run of all the tests, but it's much better.
