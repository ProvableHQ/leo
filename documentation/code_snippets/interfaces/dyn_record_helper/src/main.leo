// ANCHOR: snippet
fn get_memo(rec: dyn record) -> u64 {
    return rec.memo; // fails at runtime if `rec` does not have a field named `memo` of type `u64`
}
// ANCHOR_END: snippet

program dyn_record_helper.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}
