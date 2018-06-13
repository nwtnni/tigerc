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
good!(test_appel_18, "appel_18");
good!(test_appel_19, "appel_19");
good!(test_appel_20, "appel_20");
good!(test_appel_21, "appel_21");
good!(test_appel_22, "appel_22");
good!(test_appel_23, "appel_23");
good!(test_appel_24, "appel_24");
good!(test_appel_25, "appel_25");
good!(test_appel_26, "appel_26");
good!(test_appel_27, "appel_27");
good!(test_appel_28, "appel_28");
good!(test_appel_29, "appel_29");
good!(test_appel_30, "appel_30");
good!(test_appel_31, "appel_31");
good!(test_appel_32, "appel_32");
good!(test_appel_33, "appel_33");
good!(test_appel_34, "appel_34");
good!(test_appel_35, "appel_35");
good!(test_appel_36, "appel_36");
good!(test_appel_37, "appel_37");
good!(test_appel_38, "appel_38");
good!(test_appel_39, "appel_39");
good!(test_appel_40, "appel_40");
good!(test_appel_41, "appel_41");
good!(test_appel_42, "appel_42");
good!(test_appel_43, "appel_43");
good!(test_appel_44, "appel_44");
good!(test_appel_45, "appel_45");
good!(test_appel_46, "appel_46");
good!(test_appel_47, "appel_47");
good!(test_appel_48, "appel_48");
good!(test_appel_49, "appel_49");

good!(test_basic_add_01, "basic_add_01");
good!(test_basic_add_02, "basic_add_02");

good!(test_comment_01, "comment_01");
good!(test_comment_02, "comment_02");
good!(test_comment_03, "comment_03");
good!(test_comment_04, "comment_04");
good!(test_comment_05, "comment_05");

bad!(test_bad_var_type_01, "bad_var_type_01");
