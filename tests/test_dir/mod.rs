use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

use log::debug;

static INTEGRATION_TEST_DIR: &'static str = "testdir";

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct TestDir {
    root: PathBuf,
    dir: PathBuf,
}

impl TestDir {
    pub fn new() -> TestDir {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let mut root = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf();

        if root.ends_with("deps") {
            root.pop();
        }
        let dir = root
            .join(INTEGRATION_TEST_DIR)
            .join(&format!("test-{}", id));

        // Could error due to directory not existing
        let _ = fs::remove_dir_all(&dir);
        if let Err(err) = fs::create_dir_all(&dir) {
            panic!("Could not create '{:?}': {}", dir, err);
        }

        TestDir { root, dir }
    }

    pub fn create_file(&self, name: &str, contents: &str) {
        let path = self.path(name);

        let mut dir = path.clone();
        dir.pop();
        fs::create_dir_all(dir).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    pub fn command(&self, sub_command: &str) -> process::Command {
        let mut cmd = process::Command::new(&self.bin_path());
        cmd.current_dir(&self.dir).arg(sub_command);
        cmd
    }

    pub fn output(&self, cmd: &mut process::Command) -> process::Output {
        debug!("[{}]: {:?}", self.dir.display(), cmd);
        println!("[{}]: {:?}", self.dir.display(), cmd);
        let o = cmd.output().unwrap();
        if !o.status.success() {
            panic!(
                "\n\n===== {:?} =====\n\
                 command failed but expected success!\
                 \n\ncwd: {}\
                 \n\nstatus: {}\
                 \n\nstdout: {}\n\nstderr: {}\
                 \n\n=====\n",
                cmd,
                self.dir.display(),
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            )
        }
        o
    }

    pub fn stdout<T: FromStr>(&self, cmd: &mut process::Command) -> T {
        let o = self.output(cmd);
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout
            .trim_matches(&['\r', '\n'][..])
            .parse()
            .ok()
            .expect(&format!("Could not convert from string: '{}'", stdout))
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    pub fn bin_path(&self) -> PathBuf {
        self.root.join("rspec-timing-tool")
    }
}

impl fmt::Debug for TestDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path={}", self.dir.display())
    }
}
