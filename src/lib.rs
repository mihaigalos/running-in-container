use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn run(path: &str) -> Result<bool, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line?.contains("/docker/") {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn is_inside_docker() -> Result<bool, io::Error> {
    run("/proc/1/cgroup")
}

#[test]
fn test_detects_host_when_typical() {
    let expected = false;

    let actual = run("test/proc_1_cgroup.host").unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_detects_docker_when_typical() {
    let expected = true;

    let actual = run("test/proc_1_cgroup.docker").unwrap();

    assert_eq!(expected, actual);
}
