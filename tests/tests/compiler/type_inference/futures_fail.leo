program child.aleo {
    fn foo() -> Final {
        return final { foo_(); };
    }

    fn bar() -> Final {
        return final { bar_(); };
    }
}

final fn foo_() {}
final fn bar_() {}

// --- Next Program --- //

import child.aleo;

program type_inference.aleo {
    fn t1() -> (u64, Final) {
        let f: u32 = child.aleo/foo();
        let r: u8 = final { foo(f); };
        return (42u64, r);
    }

    fn t2() -> (u64, Final) {
        let f: (u8, u8) = child.aleo/bar();
        let r: bool = final { foo(f); };
        return (99u64, r);
    }
}

final fn foo(fut: Final) {
    fut.run();
}
