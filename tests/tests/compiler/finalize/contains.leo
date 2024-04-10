/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    mapping balances: address => u32;

    async transition foo() -> Future {
        return finalize_foo(self.caller);
    }

    async function finalize_foo(account: address) -> () {
        let expect_false: bool = Mapping::contains(balances, account);
        assert(!expect_false);
    }
}
