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
        Mapping::set(balances, account, 1u32);

        let expect_true: bool = Mapping::contains(balances, account);
        assert(expect_true);

        Mapping::remove(balances, account); // Remove the mapping entry for account

        let expect_false: bool = Mapping::contains(balances, account);
        assert(!expect_false);
    }
}
