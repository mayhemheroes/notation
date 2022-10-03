#![no_main]
use libfuzzer_sys::fuzz_target;
use notation_dsl::helper::{parse_get_tab, parse_tab};
use std::str;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        let opt = data[0] & 2;
        match str::from_utf8(&data[1..]) {
            Ok(in_string) => {
                if opt == 0 {
                    parse_tab(in_string);
                } else if opt == 1 {
                    parse_get_tab(in_string);
                }
            }
            Err(..) => (),
        }
    }
});
