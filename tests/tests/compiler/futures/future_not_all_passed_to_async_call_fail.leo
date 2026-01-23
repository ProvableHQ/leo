onchain fn finalize_foo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

onchain fn finalize_boo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

program child.aleo {
    mapping count: address => field;

    fn foo() -> Future {
        return finalize_foo(self.caller);
    }
    fn boo() -> Future {
        return finalize_boo(self.caller);
    }
 }

// --- Next Program --- //

import child.aleo;

program parent.aleo {

    fn foo() -> Future {
        let f0: Future = child.aleo/foo();
        let f1: Future = child.aleo/foo();
        let f2: Future = child.aleo/foo();
        let f3: Future = child.aleo/boo();
        let f4: Future = child.aleo/boo();
        let f5: Future = child.aleo/boo();

        // Doesn't pass f1.
        return finalize_foo(f0, f3, f2, f5, f4);
    }
}

onchain fn finalize_foo(f0: Future, f1: Future, f2: Future, f3: Future, f4: Future) {
    f1.await();
    f4.await();
    f2.await();
    f3.await();
    f0.await();
}
