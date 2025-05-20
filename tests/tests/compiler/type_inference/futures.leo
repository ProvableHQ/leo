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
        let f = child.aleo/foo();
        let r = foo(f);
        return (42u64, r);
    }

    async transition t2() -> (u64, Future) {
        let f = child.aleo/bar();
        let r = foo(f);
        return (99u64, r);
    }

    async function foo(fut: Future) {
        fut.await();
    }
}
