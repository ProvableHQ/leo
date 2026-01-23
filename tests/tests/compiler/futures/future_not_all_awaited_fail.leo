
program child.aleo {
    mapping count: address => field;

    fn foo() -> Future {
        return finalize_foo(self.caller);
    }
    fn boo() -> Future {
        return finalize_boo(self.caller);
    }
 }

onchain fn finalize_foo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

onchain fn finalize_boo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn foo() -> Future {
        return finalize_foo(child.aleo/foo(), child.aleo/foo(), child.aleo/foo(), child.aleo/boo(), child.aleo/boo(), child.aleo/boo());
    }
}

// Doesn't await f4.
onchain fn finalize_foo(f0: Future, f1: Future, f2: Future, f3: Future, f4: Future, f5: Future) {
    f1.await();
    f2.await();
    f3.await();
    f0.await();
    f5.await();
}
