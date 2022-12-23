#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use notation_dsl::helper::{parse_get_tab, parse_tab};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    choice: bool,
    str: String,
}

fuzz_target!(|data: FuzzInput| {
    if data.choice {
        let _ = parse_tab(&data.str);
    } else {
        let _ = parse_get_tab(&data.str);
    }
});
