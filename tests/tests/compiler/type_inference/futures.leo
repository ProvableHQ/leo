program child.aleo {
    fn foo() -> Final {
        return foo_();
    }
    fn bar() -> Final {
        return bar_();
    }
}

onchain fn foo_() {}
onchain fn bar_() {}

// --- Next Program --- //

import child.aleo;

program type_inference.aleo {
    fn t1() -> (u64, Final) {
        let f = child.aleo/foo();
        let r = foo(f);
        return (42u64, r);
    }

    fn t2() -> (u64, Final) {
        let f = child.aleo/bar();
        let r = foo(f);
        return (99u64, r);
    }
}

onchain fn foo(fut: Final) {
    fut.run();
}
