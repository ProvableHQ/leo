
program child.aleo {
    fn foo() -> Future {
        return finalize_foo(0u32);
    }
    fn boo() -> (u32, Future) {
        return (1u32, finalize_boo(0u32));
    }
 }

onchain fn finalize_foo(x: u32) {
    assert_eq(1u32, 1u32);
}

onchain fn finalize_boo(x: u32) {
    assert_eq(1u32, 1u32);
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {

    fn foo() -> Future {
        let f0: Future = child.aleo/foo();

        child.aleo/foo();

        child.aleo/boo();

        return finalize_foo(f0);
    }
}

onchain fn finalize_foo(f0: Future) {
    f0.await();
}
