
program child.aleo {
    fn foo() -> Final {
        return finalize_foo(0u32);
    }
    fn boo() -> (u32, Final) {
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

    fn foo() -> Final {
        let f0: Final = child.aleo/foo();

        child.aleo/foo();

        child.aleo/boo();

        return finalize_foo(f0);
    }
}

onchain fn finalize_foo(f0: Final) {
    f0.run();
}
