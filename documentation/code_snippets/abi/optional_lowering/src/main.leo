// ANCHOR: process
fn process(value: u32?) -> u32 {
    return value.unwrap_or(0u32);
}
// ANCHOR_END: process

program abi_optional.aleo {
    fn run(input: u32) -> u32 {
        return process(input);
    }

    fn nested_optional_demo() -> u64 {
        // ANCHOR: nested_optional
        let arr: [u64?; 2] = [1u64, none];
        // ANCHOR_END: nested_optional
        return arr[0].unwrap_or(0u64);
    }

    @noupgrade
    constructor() {}
}
