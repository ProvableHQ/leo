program child.aleo {
    async fn foo() -> Future {
        return foo_();
    }
    async fn bar() -> Future {
        return bar_();
    }
}

async fn foo_() {}
async fn bar_() {}

// --- Next Program --- //

import child.aleo;

program type_inference.aleo {
    async fn t1() -> (u64, Future) {
        let f = child.aleo/foo();
        let r = foo(f);
        return (42u64, r);
    }

    async fn t2() -> (u64, Future) {
        let f = child.aleo/bar();
        let r = foo(f);
        return (99u64, r);
    }
}

async fn foo(fut: Future) {
    fut.await();
}
