program test.aleo {
    fn foo() -> Final {
        return final { finalize_foo(); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_foo() {
    let _a: address = ChaCha::rand_address();
    let b: bool = ChaCha::rand_bool();
    let _c: field = ChaCha::rand_field();
    let _d: group = ChaCha::rand_group();
    let _e: i8 = ChaCha::rand_i8();
    let _f: i16 = ChaCha::rand_i16();
    let _g: i32 = ChaCha::rand_i32();
    let _h: i64 = ChaCha::rand_i64();
    let _i: i128 = ChaCha::rand_i128();
    let _j: scalar = ChaCha::rand_scalar();
    let _k: u8 = ChaCha::rand_u8();
    let _l: u16 = ChaCha::rand_u16();
    let _m: u32 = ChaCha::rand_u32();
    let _n: u64 = ChaCha::rand_u64();
    let _o: u128 = ChaCha::rand_u128();
    assert(b);
}
