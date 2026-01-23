/*
seed = 987654321
min_height = 16

[case]
program = "unwrap_none_test.aleo"
function = "unwrap_none"
input = []

[case]
program = "unwrap_none_test.aleo"
function = "unwrap_struct_field_none"
input = []
*/

// === Struct with optional field ===
struct MaybeValue {
    val: u64?,
}

program unwrap_none_test.aleo {
    // === Direct unwrap of a none value ===
    fn unwrap_none() {
        let x: u8? = none;
        assert(x.unwrap() > 0); // ❌ should fail
    }

    fn unwrap_struct_field_none() {
        let s: MaybeValue = MaybeValue { val: none };
        assert(s.val.unwrap() > 0); // ❌ should also fail
    }

    @noupgrade
    constructor() {}
}
