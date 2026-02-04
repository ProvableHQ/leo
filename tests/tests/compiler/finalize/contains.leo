
program test.aleo {

    mapping balances: address => u32;

    fn foo() -> Final {
        return finalize_foo(self.caller);
    }
}

final fn finalize_foo(account: address) -> () {
    let expect_false: bool = Mapping::contains(balances, account);
    assert(!expect_false);
}
