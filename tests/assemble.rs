#[macro_use]
mod util;

use std::fs::remove_file;

use util::*;

generate!(good, "type", "-t", "typedsol", "typed", compare_content);

generate!(bad, "type", "-t", "typedsol", "typed", compare_location);
