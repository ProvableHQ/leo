
program test.aleo {
    fn matches(addr: address) -> (bool, Final) {
        return (self.caller == addr, finalize_matches(self.caller));
    }
}

final fn finalize_matches(addr: address) {
    assert_eq(addr, self.caller);
}
