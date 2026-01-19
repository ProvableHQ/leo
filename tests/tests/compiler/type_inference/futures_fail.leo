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
        let f: u32 = child.aleo/foo();
        let r: u8 = foo(f);
        return (42u64, r);
    }

    async fn t2() -> (u64, Future) {
        let f: (u8, u8) = child.aleo/bar();
        let r: bool = foo(f);
        return (99u64, r);
    }
}

async fn foo(fut: Future) {
    fut.await();
}
