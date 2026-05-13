// ANCHOR: snippet
const CEILING: u32 = math_utils::MAX_U32;
// ANCHOR_END: snippet

program ceiling_demo.aleo {
    fn use_ceiling() -> u32 {
        return CEILING;
    }

    @noupgrade
    constructor() {}
}
