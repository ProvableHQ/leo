// ANCHOR: struct
struct Array3 {
    a0: u32,
    a1: u32,
    a2: u32,
}
// ANCHOR_END: struct

program structure_decls.aleo {
    // ANCHOR: mapping
    // On-chain storage of an `account` mapping,
    // with `address` as the type of keys,
    // and `u64` as the type of values.
    mapping account: address => u64;
    // ANCHOR_END: mapping

    // ANCHOR: storage_var
    // On-chain storage of an `counter` storage variable of type u32,
    storage counter: u32;
    // ANCHOR_END: storage_var

    // ANCHOR: storage_vec
    // On-chain storage of an `accounts` storage vector of type address,
    storage accounts: [address];
    // ANCHOR_END: storage_vec

    fn ping() {}

    @noupgrade
    constructor() {}
}
