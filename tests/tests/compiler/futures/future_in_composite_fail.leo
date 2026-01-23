
struct S {
    member: Future,
}

program test.aleo {
    mapping x: Future => Future;
    fn main() -> Future {
        let future: Future = finish();
        let an_array: [Future; 1] = [future];
        return an_array[0u32];
    }
}

onchain fn finish() {
    assert_eq(1u8, 1u8);
}
