
program test.aleo {
    fn matches(addr: address) -> (bool, Final) {
        return (self.caller == addr, finalize_matches(self.caller));
    }
}

onchain fn finalize_matches(addr: address) {
    assert_eq(addr, self.caller);
}
