
program test.aleo {

    mapping balances: address => u32;

    fn foo() -> Final {
        let caller = self.caller;
        return final { finalize_foo(caller); };
    }
}

final fn finalize_foo(account: address) -> () {
    let expect_false: bool = Mapping::contains(balances, account);
    assert(!expect_false);
}
