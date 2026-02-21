
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
        let f0: Final = child.aleo/foo();
        let f1: Final = child.aleo/foo();
        let f2: Final = child.aleo/foo();
        let f3: Final = child.aleo/boo();
        let f4: Final = child.aleo/boo();
        let f5: Final = child.aleo/boo();

        // Doesn't capture f1.
        return final {
            f0.run(); 
            f3.run(); 
            f2.run(); 
            f5.run(); 
            f4.run();
        };
    }
}
