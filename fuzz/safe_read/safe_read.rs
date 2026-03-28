#![no_main]

use libfuzzer_sys::fuzz_target;
use ai_moxcms::ColorProfile;

fuzz_target!(|data: &[u8]| {
    // Never panic expected
    _ = ColorProfile::new_from_slice(data);
});
