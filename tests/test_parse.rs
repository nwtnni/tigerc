extern crate regex;

use std::io::prelude::*;
use std::fs::{File, remove_file};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct Unit {
    file: PathBuf,
    expected: PathBuf,
    actual: PathBuf,
}

fn read_to_string(file: &PathBuf) -> String {
	let mut f = File::open(file).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

fn get_unit(name: &str, directory: &str) -> Unit {
    let dir = PathBuf::from(file!()).parent().unwrap().join(directory);
    let file = dir.join(PathBuf::from(name).with_extension("tig"));
    let expected = file.with_extension("parsedsol");
    let actual = file.with_extension("parsed");
    Unit { file, expected, actual }
}

fn run_parse(file: &PathBuf) {
    Command::new("target/debug/tigerc")
        .arg("--parse")
        .arg(file)
        .output()
        .unwrap();
}

fn get_location(text: String) -> (usize, usize) {
    let re = Regex::new(r"^(\d+):(\d+) ").unwrap();
    let caps = re.captures(&text).unwrap();
    (usize::from_str(&caps[1]).unwrap(), usize::from_str(&caps[2]).unwrap())
}

macro_rules! good {
    ($name:ident, $file:expr) => {
        #[test]
        pub fn $name() {
            let Unit { file, expected, actual } = get_unit($file, "parse");
            run_parse(&file);
            let exp = read_to_string(&expected);
            let act = read_to_string(&actual);
            assert_eq!(
                exp.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
                act.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
            );
            remove_file(actual).unwrap();
        }
    }
}

macro_rules! bad {
    ($name:ident, $file:expr) => {
        #[test]
        pub fn $name() {
            let Unit { file, expected, actual } = get_unit($file, "parse");
            run_parse(&file);
            let exp = read_to_string(&expected);
            let act = read_to_string(&actual);
            assert_eq!(get_location(exp), get_location(act));
            remove_file(actual).unwrap();
        }
    }
}

good!(test_basic_add_01, "basic_add_01");
good!(test_basic_add_02, "basic_add_02");

bad!(test_bad_var_type_01, "bad_var_type_01");
