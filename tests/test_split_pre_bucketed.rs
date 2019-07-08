use std::process;

mod test_dir;
use test_dir::TestDir;

fn setup_test() -> TestDir {
    let test_dir = TestDir::new();
    test_dir.create_file("spec/tests_a_spec.rb", "_");
    test_dir.create_file("spec/tests_b_spec.rb", "_");
    test_dir.create_file("spec/test_helper.rb", "_");
    test_dir.create_file("spec/nested/tests_c_spec.rb", "_");
    test_dir.create_file("spec/nested/tests_d_spec.rb", "_");
    test_dir.create_file("spec/nested/tests_e_spec.rb", "_");

    test_dir.create_file(
        "rspec-timings.txt",
        r###"
[
    [
        ["./spec/tests_a_spec.rb", 524]
    ],
    [
        ["./spec/nested/tests_c_spec.rb", 2.4213],
        ["./spec/nested/tests_e_spec.rb", 5]
    ]
]
    "###,
    );

    test_dir
}

fn split_with_current_split(test_dir: &TestDir, current_split: u32) -> process::Command {
    let mut cmd = test_dir.command("split-pre-bucketed");
    cmd.arg("-s");
    cmd.arg("4");

    cmd.arg("-c");
    cmd.arg(current_split.to_string());

    cmd.arg(test_dir.path("rspec-timings.txt"));
    cmd
}

fn all_splits(test_dir: &TestDir) -> Vec<Vec<String>> {
    let mut all_splits = vec![];
    for i in 0..4 {
        all_splits.push(
            test_dir
                .stdout::<String>(&mut split_with_current_split(&test_dir, i))
                .split(" ")
                .map(|v| v.to_owned())
                .collect::<Vec<_>>(),
        )
    }
    all_splits
}

fn search_splits(all_splits: &Vec<Vec<String>>, search_file: &str) -> bool {
    for split in all_splits {
        for file in split {
            if file.contains(search_file) {
                return true;
            }
        }
    }

    return false;
}

#[test]
fn it_covers_all_test_files() {
    let test_dir = setup_test();
    let all_splits = all_splits(&test_dir);
    assert_eq!(search_splits(&all_splits, "spec/tests_a_spec.rb"), true);
    assert_eq!(search_splits(&all_splits, "spec/tests_b_spec.rb"), true);
    assert_eq!(
        search_splits(&all_splits, "spec/nested/tests_c_spec.rb"),
        true
    );
    assert_eq!(
        search_splits(&all_splits, "spec/nested/tests_d_spec.rb"),
        true
    );
    assert_eq!(
        search_splits(&all_splits, "spec/nested/tests_e_spec.rb"),
        true
    );
}
