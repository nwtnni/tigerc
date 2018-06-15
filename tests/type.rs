#[macro_use]
mod util;

use std::fs::remove_file;

use util::*;

generate!(good, "type", "-t", "typedsol", "typed", |exp: String, act: String| {
    assert_eq!(
        exp.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
        act.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
    )
});

generate!(bad, "type", "-t", "typedsol", "typed", |exp: String, act: String| {
    assert_eq!(get_location(exp), get_location(act))
});

good!(test_queens, "queens");
good!(test_merge, "merge");

good!(test_appel_01, "appel_01");
good!(test_appel_02, "appel_02");
good!(test_appel_03, "appel_03");
good!(test_appel_04, "appel_04");
good!(test_appel_05, "appel_05");
good!(test_appel_06, "appel_06");
good!(test_appel_07, "appel_07");
good!(test_appel_08, "appel_08");
good!(test_appel_12, "appel_12");
good!(test_appel_27, "appel_27");
good!(test_appel_30, "appel_30");
good!(test_appel_37, "appel_37");
good!(test_appel_41, "appel_41");
good!(test_appel_42, "appel_42");
good!(test_appel_44, "appel_44");
good!(test_appel_46, "appel_46");
good!(test_appel_47, "appel_47");
good!(test_appel_48, "appel_48");
good!(test_appel_49, "appel_49");

bad!(test_appel_09, "appel_09");
bad!(test_appel_10, "appel_10");
bad!(test_appel_11, "appel_11");
bad!(test_appel_13, "appel_13");
bad!(test_appel_14, "appel_14");
bad!(test_appel_15, "appel_15");
// bad!(test_appel_16, "appel_16");
bad!(test_appel_17, "appel_17");
bad!(test_appel_18, "appel_18");
bad!(test_appel_19, "appel_19");
bad!(test_appel_20, "appel_20");
bad!(test_appel_21, "appel_21");
bad!(test_appel_22, "appel_22");
bad!(test_appel_23, "appel_23");
bad!(test_appel_24, "appel_24");
bad!(test_appel_25, "appel_25");
bad!(test_appel_26, "appel_26");
bad!(test_appel_28, "appel_28");
bad!(test_appel_29, "appel_29");
bad!(test_appel_31, "appel_31");
bad!(test_appel_32, "appel_32");
bad!(test_appel_33, "appel_33");
bad!(test_appel_34, "appel_34");
bad!(test_appel_35, "appel_35");
bad!(test_appel_36, "appel_36");
bad!(test_appel_38, "appel_38");
bad!(test_appel_39, "appel_39");
bad!(test_appel_40, "appel_40");
bad!(test_appel_43, "appel_43");
bad!(test_appel_45, "appel_45");
