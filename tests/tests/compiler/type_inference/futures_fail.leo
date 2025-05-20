program child.aleo {
    async transition foo() -> Future {
        return foo_();
    }

    async function foo_() {}

    async transition bar() -> Future {
        return bar_();
    }

    async function bar_() {}
}

// --- Next Program --- //

import child.aleo;

program type_inference.aleo {
    async transition t1() -> (u64, Future) {
        let f: u32 = child.aleo/foo();
        let r: u8 = foo(f);
        return (42u64, r);
    }

    async transition t2() -> (u64, Future) {
        let f: (u8, u8) = child.aleo/bar();
        let r: bool = foo(f);
        return (99u64, r);
    }

    async function foo(fut: Future) {
        fut.await();
    }
}
