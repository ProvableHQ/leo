program child.aleo {
    mapping foo: u32 => u32; 

    fn bar() { }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    mapping f: u32 => u32;

    fn set_test() -> Final {
        return final {
            for i in 0u32..2u32 {
                f.set(i, 0);
            }
        };
    }

    fn get_test() -> Final {
        return final {
            for i in 0u32..2u32 {
                let foo = child.aleo/foo.get(i);
                assert(foo == 42);
            }
        };
    }

    fn get_or_use_test() -> Final {
        return final {
            for i in 0u32..2u32 {
                let foo = child.aleo/foo.get_or_use(i, 42);
                assert(foo == 42);
            }
        };
    }
}

