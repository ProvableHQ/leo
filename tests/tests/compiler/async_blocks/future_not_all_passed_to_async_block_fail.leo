
program child.aleo {
    mapping count: address => field;

    async transition foo() -> Future {
        return finalize_foo(self.caller);
    }

    async function finalize_foo(addr: address) {
        let val: field = Mapping::get_or_use(count, addr, 0field);
        Mapping::set(count, addr, val + 1field);
    }

    async transition boo() -> Future {
        return finalize_boo(self.caller);
    }

    async function finalize_boo(addr: address) {
        let val: field = Mapping::get_or_use(count, addr, 0field);
        Mapping::set(count, addr, val + 1field);
    }
 }

// --- Next Program --- //

import child.aleo;

program parent.aleo {

    async transition foo() -> Future {
        let f0: Future = child.aleo/foo();
        let f1: Future = child.aleo/foo();
        let f2: Future = child.aleo/foo();
        let f3: Future = child.aleo/boo();
        let f4: Future = child.aleo/boo();
        let f5: Future = child.aleo/boo();

        // Doesn't capture f1.
        return async {
            f0.await(); 
            f3.await(); 
            f2.await(); 
            f5.await(); 
            f4.await();
        };
    }
}
