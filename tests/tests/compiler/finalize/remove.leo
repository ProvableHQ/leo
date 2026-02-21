
program test.aleo {

    mapping balances: address => u32;

    fn foo() -> Final {
        let caller = self.caller;
        return final { finalize_foo(caller); };
    }
}

final fn finalize_foo(account: address) -> () {
    Mapping::set(balances, account, 1u32);

    let expect_true: bool = Mapping::contains(balances, account);
    assert(expect_true);

    Mapping::remove(balances, account); // Remove the mapping entry for account

    let expect_false: bool = Mapping::contains(balances, account);
    assert(!expect_false);
}
