
program test.aleo {

    mapping balances: address => u32;

    async fn foo() -> Future {
        return finalize_foo(self.caller);
    }
}

async fn finalize_foo(account: address) -> () {
    let expect_false: bool = Mapping::contains(balances, account);
    assert(!expect_false);
}
