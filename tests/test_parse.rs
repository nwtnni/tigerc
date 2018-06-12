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

good!(test_appel_01, "appel_01");
good!(test_appel_02, "appel_02");
good!(test_appel_03, "appel_03");
good!(test_appel_04, "appel_04");
good!(test_appel_05, "appel_05");
good!(test_appel_06, "appel_06");
good!(test_appel_07, "appel_07");
good!(test_appel_08, "appel_08");
good!(test_appel_09, "appel_09");
good!(test_appel_10, "appel_10");
good!(test_appel_11, "appel_11");
good!(test_appel_12, "appel_12");
good!(test_appel_13, "appel_13");
good!(test_appel_14, "appel_14");
good!(test_appel_15, "appel_15");
good!(test_appel_16, "appel_16");
good!(test_appel_17, "appel_17");

good!(test_basic_add_01, "basic_add_01");
good!(test_basic_add_02, "basic_add_02");

good!(test_comment_01, "comment_01");
good!(test_comment_02, "comment_02");
good!(test_comment_03, "comment_03");
good!(test_comment_04, "comment_04");
good!(test_comment_05, "comment_05");

bad!(test_bad_var_type_01, "bad_var_type_01");
