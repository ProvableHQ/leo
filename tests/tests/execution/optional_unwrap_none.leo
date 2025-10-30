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

// --- DO NOT RUN THIS TEST JUST YET --- //

import unwrap_none_test.aleo;

program test_unwrap_none_test.aleo {
    @test
    @should_fail
    transition unwrap_none() {
        unwrap_none_test.aleo/unwrap_none();
    }

    @test
    @should_fail
    transition unwrap_struct_field_none() {
        unwrap_none_test.aleo/unwrap_struct_field_none();
    }

    @noupgrade
    async constructor() {}
}

