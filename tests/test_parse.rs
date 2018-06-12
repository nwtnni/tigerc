#[macro_use]
mod util;

use std::fs::remove_file;

use util::*;

generate!(good, "parse", "-p", "parsedsol", "parsed", |exp: String, act: String| {
    assert_eq!(
        exp.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
        act.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
    )
});

generate!(bad, "parse", "-p", "parsedsol", "parsed", |exp: String, act: String| {
    assert_eq!(get_location(exp), get_location(act))
});

good!(test_basic_add_01, "basic_add_01");
good!(test_basic_add_02, "basic_add_02");

bad!(test_bad_var_type_01, "bad_var_type_01");
