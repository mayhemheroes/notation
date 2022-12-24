#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = notation_dsl::helper::parse_tab(data);
});
