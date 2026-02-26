
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

final fn finalize_foo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

final fn finalize_boo(addr: address) {
    let val: field = Mapping::get_or_use(count, addr, 0field);
    Mapping::set(count, addr, val + 1field);
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn foo() -> Final {
        let f1 = child.aleo/foo();
        let f2 = child.aleo/foo();
        let f3 = child.aleo/foo();
        let f4 = child.aleo/boo();
        let f5 = child.aleo/boo();
        let f6 = child.aleo/boo();
        return final { finalize_foo(f1, f2, f3, f4, f5, f6); };
    }
}

// Doesn't await f4.
final fn finalize_foo(f0: Final, f1: Final, f2: Final, f3: Final, f4: Final, f5: Final) {
    f1.run();
    f2.run();
    f3.run();
    f0.run();
    f5.run();
}
