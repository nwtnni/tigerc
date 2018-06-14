#[macro_use]
mod util;

use std::fs::remove_file;

use util::*;

generate!(good, "lex", "-l", "lexedsol", "lexed", |exp: String, act: String| {
    assert_eq!(
        exp.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
        act.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
    )
});

generate!(bad, "lex", "-l", "lexedsol", "lexed", |exp: String, act: String| {
    assert_eq!(get_location(exp), get_location(act))
});

good!(test_ident_01, "ident_01");
good!(test_ident_02, "ident_02");
good!(test_ident_03, "ident_03");
good!(test_ident_04, "ident_04");

bad!(test_bad_ident_01, "bad_ident_01");
bad!(test_bad_ident_02, "bad_ident_02");

good!(test_keyword_01, "keyword_01");
good!(test_keyword_02, "keyword_02");
good!(test_keyword_03, "keyword_03");
good!(test_keyword_04, "keyword_04");
good!(test_keyword_05, "keyword_05");
good!(test_keyword_06, "keyword_06");
good!(test_keyword_07, "keyword_07");
good!(test_keyword_08, "keyword_08");
good!(test_keyword_09, "keyword_09");
good!(test_keyword_10, "keyword_10");
good!(test_keyword_11, "keyword_11");
good!(test_keyword_12, "keyword_12");
good!(test_keyword_13, "keyword_13");
good!(test_keyword_14, "keyword_14");
good!(test_keyword_15, "keyword_15");
good!(test_keyword_16, "keyword_16");
good!(test_keyword_17, "keyword_17");

good!(test_operator_01, "operator_01");
good!(test_operator_02, "operator_02");
good!(test_operator_03, "operator_03");
good!(test_operator_04, "operator_04");
good!(test_operator_05, "operator_05");
good!(test_operator_06, "operator_06");
good!(test_operator_07, "operator_07");
good!(test_operator_08, "operator_08");
good!(test_operator_09, "operator_09");
good!(test_operator_10, "operator_10");
good!(test_operator_11, "operator_11");
good!(test_operator_12, "operator_12");
good!(test_operator_13, "operator_13");
good!(test_operator_14, "operator_14");

good!(test_symbol_01, "symbol_01");
good!(test_symbol_02, "symbol_02");
good!(test_symbol_03, "symbol_03");
good!(test_symbol_04, "symbol_04");
good!(test_symbol_05, "symbol_05");
good!(test_symbol_06, "symbol_06");
good!(test_symbol_07, "symbol_07");
good!(test_symbol_08, "symbol_08");
good!(test_symbol_09, "symbol_09");
