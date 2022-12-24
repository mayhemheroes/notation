#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    notation_dsl::helper::parse_tab(data);
});
