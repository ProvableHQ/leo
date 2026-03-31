// --- library: mixed_lib --- //

const BASE: u32 = 10u32;

// --- Next Module: ops.leo --- //
fn scale(x: u32) -> u32 {
    return x * mixed_lib::BASE;
}

fn offset(x: u32) -> u32 {
    return x + mixed_lib::BASE;
}

// --- Next Program --- //

program submodule_and_top_level.aleo {
    fn main() -> u32 {
        let scaled: u32 = mixed_lib::ops::scale(3u32);
        let shifted: u32 = mixed_lib::ops::offset(mixed_lib::BASE);
        return scaled + shifted;
    }
}
