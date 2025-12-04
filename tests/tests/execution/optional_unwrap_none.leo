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

program unwrap_none_test.aleo {
    // === Direct unwrap of a none value ===
    transition unwrap_none() {
        let x: u8? = none;
        assert(x.unwrap() > 0); // ❌ should fail
    }

    // === Struct with optional field ===
    struct MaybeValue {
        val: u64?,
    }

    transition unwrap_struct_field_none() {
        let s: MaybeValue = MaybeValue { val: none };
        assert(s.val.unwrap() > 0); // ❌ should also fail
    }

    @noupgrade
    async constructor() {}
}
