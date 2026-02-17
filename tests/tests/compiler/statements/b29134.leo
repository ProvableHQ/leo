program child.aleo {
    mapping foo: u32 => u32; 

    transition bar() { }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    mapping f: u32 => u32;

    async transition set_test() -> Future {
        return async {
            for i in 0u32..2u32 {
                f.set(i, 0);
            }
        };
    }

    async transition get_test() -> Future {
        return async {
            for i in 0u32..2u32 {
                let foo = child.aleo/foo.get(i);
                assert(foo == 42);
            }
        };
    }

    async transition get_or_use_test() -> Future {
        return async {
            for i in 0u32..2u32 {
                let foo = child.aleo/foo.get_or_use(i, 42);
                assert(foo == 42);
            }
        };
    }
}

