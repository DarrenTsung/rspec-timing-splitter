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

    test_dir.create_file(
        "rspec-timings.txt",
        r###"
    [
        {"file_path":"./spec/tests_a_spec.rb","total_time":3.3},
        {"file_path":"./spec/nested/tests_c_spec.rb","total_time":31.903082000000023}
    ]
    "###,
    );

    test_dir
}

fn split_with_current_split(test_dir: &TestDir, current_split: u32) -> process::Command {
    let mut cmd = test_dir.command("split");
    cmd.arg("-s");
    cmd.arg("4");

    cmd.arg("-c");
    cmd.arg(current_split.to_string());

    cmd.arg(test_dir.path("rspec-timings.txt"));
    cmd
}

fn all_splits(test_dir: &TestDir) -> Vec<Vec<String>> {
    vec![
        test_dir
            .stdout::<String>(&mut split_with_current_split(&test_dir, 0))
            .split(" ")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>(),
        test_dir
            .stdout::<String>(&mut split_with_current_split(&test_dir, 1))
            .split(" ")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>(),
        test_dir
            .stdout::<String>(&mut split_with_current_split(&test_dir, 2))
            .split(" ")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>(),
        test_dir
            .stdout::<String>(&mut split_with_current_split(&test_dir, 3))
            .split(" ")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>(),
    ]
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
}
