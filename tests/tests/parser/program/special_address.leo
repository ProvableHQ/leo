/*
namespace: Parse
expectation: Pass
*/

program test.aleo {
    mapping Yo: address => u32;

    transition main(a: address) {
        assert_eq(a, self.caller);
        assert_eq(a, self.address);
        assert_eq(a, hello.aleo);
        return foo.aleo;
    }
}