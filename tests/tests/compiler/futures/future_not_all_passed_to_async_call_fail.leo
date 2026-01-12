final fn finalize_foo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

final fn finalize_boo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

program child.aleo {
    mapping count: address => field;

    fn foo() -> Final {
        let caller = self.caller;
        return final { finalize_foo(caller); };
    }
    fn boo() -> Final {
        let caller = self.caller;
        return final { finalize_boo(caller); };
    }
 }

// --- Next Program --- //

import child.aleo;

program parent.aleo {

    fn foo() -> Final {
        let f0: Final = child.aleo/foo();
        let f1: Final = child.aleo/foo();
        let f2: Final = child.aleo/foo();
        let f3: Final = child.aleo/boo();
        let f4: Final = child.aleo/boo();
        let f5: Final = child.aleo/boo();

        // Doesn't pass f1.
        return final { finalize_foo(f0, f3, f2, f5, f4); };
    }
}

final fn finalize_foo(f0: Final, f1: Final, f2: Final, f3: Final, f4: Final) {
    f1.run();
    f4.run();
    f2.run();
    f3.run();
    f0.run();
}
