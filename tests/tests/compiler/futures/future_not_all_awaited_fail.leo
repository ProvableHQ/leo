/*
namespace: Compile
expectation: Fail
*/

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
        return finalize_foo(child.aleo/foo(), child.aleo/foo(), child.aleo/foo(), child.aleo/boo(), child.aleo/boo(), child.aleo/boo());
    }

    // Doesn't await f4.
    async function finalize_foo(f0: Future, f1: Future, f2: Future, f3: Future, f4: Future, f5: Future) {
        f1.await();
        f2.await();
        f3.await();
        f0.await();
        f5.await();
    }
}