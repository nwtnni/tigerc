extern crate regex;

use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use self::regex::Regex;

/// Generates a test-generating macro.
///
/// [$macro]   is the name of the test generator
/// [$dir]     is the subdirectory of [tests] to look for files in
/// [$flag]    is the command line flag to pass to the compiler
/// [$sol_ext] is the file extension of the solution file
/// [$act_ext] is the file extension of the generated file
/// [$compare] is the function used to compare the solution and generated files as Strings
macro_rules! generate {
    ($macro:ident, $dir:expr, $flag:expr, $sol_ext:expr, $act_ext:expr, $compare:expr) => {

        /// Test-generating macro.
        /// [$name] is the name of the generated test function
        /// [$file] is the name of the test file, without extensions
        macro_rules! $macro {
            ($name:ident, $file:expr) => {
                #[test]
                pub fn $name() {
                    let Unit { file, solution, actual } = get_unit($file, $dir, $sol_ext, $act_ext);
                    run($flag, &file);
                    let exp = read_to_string(&solution);
                    let act = read_to_string(&actual);
                    remove_file(actual).unwrap();
                    $compare(exp, act);
                }
            }
        }

    }
}

/// Single test unit
pub struct Unit {

    /// Path to test .tig file
    pub file: PathBuf,

    /// Path to solution
    pub solution: PathBuf,

    /// Path to generated diagnostic file
    pub actual: PathBuf,
}

/// Read the contents of the file into a String
pub fn read_to_string(path: &PathBuf) -> String {
	let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Return the compilation unit
pub fn get_unit(name: &str, directory: &str, sol_ext: &str, act_ext: &str) -> Unit {
    let dir = current_dir().unwrap().join("tests").join(directory);
    let file = dir.join(PathBuf::from(name).with_extension("tig"));
    let solution = file.with_extension(sol_ext);
    let actual = file.with_extension(act_ext);
    Unit { file, solution, actual }
}

/// Run the compiler with the given flag
pub fn run(arg: &str, file: &PathBuf) {
    Command::new("target/debug/tigerc")
        .arg(arg)
        .arg(file)
        .output()
        .unwrap();
}

/// Return the error locations in a String
pub fn get_location(text: String) -> (usize, usize) {
    let re = Regex::new(r"^(\d+):(\d+) ").unwrap();
    let caps = re.captures(&text).unwrap();
    (usize::from_str(&caps[1]).unwrap(), usize::from_str(&caps[2]).unwrap())
}
