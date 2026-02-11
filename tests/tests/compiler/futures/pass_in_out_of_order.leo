program test.aleo {
    mapping foo: u32 => u32;
    fn main_inner(public a: u32, public b: u32) -> Final {
        return final { finalize(a, b); };
    }
}

final fn finalize(a: u32, b: u32) {
    Mapping::set(foo, a, b);
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    fn main(public a: u32, b: u32) -> Final {
        let f1: Final = test.aleo/main_inner(0u32, 0u32);
        let f2: Final = test.aleo/main_inner(1u32, 1u32);
        let f:Final = final { finalize(f2, f1); };
        return f;
    }
}

final fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}
