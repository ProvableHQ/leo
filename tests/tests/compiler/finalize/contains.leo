/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    mapping balances: address => u32;

    transition foo() -> () {
        return then finalize(self.caller);
    }

    finalize foo(account: address) -> () {
        let expect_false: bool = Mapping::contains(balances, account);
        assert(!expect_false);
    }
}
