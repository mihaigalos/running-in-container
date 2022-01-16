use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_inside_docker(path: &str) -> Result<bool, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line?.contains("/docker/") {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn run() -> Result<bool, io::Error> {
    is_inside_docker("/proc/1/cgroup")
}

#[test]
fn test_detects_host_when_typical() {
    let expected = false;

    let actual = is_inside_docker("test/proc_1_cgroup.host").unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_detects_docker_when_typical() {
    let expected = true;

    let actual = is_inside_docker("test/proc_1_cgroup.docker").unwrap();

    assert_eq!(expected, actual);
}
