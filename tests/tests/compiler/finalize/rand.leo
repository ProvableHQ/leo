/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    transition foo() {
        return then finalize();
    }

    finalize foo() {
        let a: address = ChaCha::rand_address();
        let b: bool = ChaCha::rand_bool();
        let c: field = ChaCha::rand_field();
        let d: group = ChaCha::rand_group();
        let e: i8 = ChaCha::rand_i8();
        let f: i16 = ChaCha::rand_i16();
        let g: i32 = ChaCha::rand_i32();
        let h: i64 = ChaCha::rand_i64();
        let i: i128 = ChaCha::rand_i128();
        let j: scalar = ChaCha::rand_scalar();
        let k: u8 = ChaCha::rand_u8();
        let l: u16 = ChaCha::rand_u16();
        let m: u32 = ChaCha::rand_u32();
        let n: u64 = ChaCha::rand_u64();
        let o: u128 = ChaCha::rand_u128();
        assert(b);
    }
}
