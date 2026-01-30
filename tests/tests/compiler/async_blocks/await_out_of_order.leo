program test.aleo {
    mapping foo: u32 => u32;
    fn main_inner(public a: u32, public b: u32) -> Final {
        return final {
            Mapping::set(foo, a, b);
        };
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    fn main(public a: u32, b: u32) -> Final {
        let f1: Final = test.aleo/main_inner(0u32, 0u32);
        let f2: Final = test.aleo/main_inner(1u32, 1u32);

        // No "out of order" warning since the futures are captured in the order they appear in inside the block anyways.
        let f:Final = final {
            f2.run();
            f1.run();
        };
        return f;
    }
}
