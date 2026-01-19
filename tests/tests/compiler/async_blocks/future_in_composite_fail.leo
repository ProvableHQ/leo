struct S {
    member: Future,
}

program test.aleo {
    mapping x: Future => Future;
    async fn main() -> Future {
        let future = async {
            assert_eq(1u8, 1u8);
        };
        let an_array: [Future; 1] = [future];
        return an_array[0u32];
    }
}
