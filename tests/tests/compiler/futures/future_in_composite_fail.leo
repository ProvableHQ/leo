/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    mapping x: Future => Future;

    struct S {
        member: Future,
    }

    async transition main() -> Future {
        let future: Future = finish();
        let an_array: [Future; 1] = [future];
        return an_array[0u32];
    }

    async function finish() {
        assert_eq(1u8, 1u8);
    }
}
