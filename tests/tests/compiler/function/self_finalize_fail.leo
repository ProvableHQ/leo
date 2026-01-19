
program test.aleo {
    async fn matches(addr: address) -> (bool, Future) {
        return (self.caller == addr, finalize_matches(self.caller));
    }
}

async fn finalize_matches(addr: address) {
    assert_eq(addr, self.caller);
}
