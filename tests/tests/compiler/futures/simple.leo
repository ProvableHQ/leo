program test.aleo {
    mapping foo: u32 => u32;
    fn main_inner(public a: u32, b: u32) -> (u32, Final) {
        let c: u32 = a + b;
        let f: Final = final { finalize(); };
        return (c, f);
    }
}

final fn finalize() {
    Mapping::set(foo, 1u32, 1u32);
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final) {
        let c: u32 = a + b;
        let (d, f1): (u32, Final) = test.aleo/main_inner(1u32, 1u32);
        let f:Final = final { finalize(c, f1); };
        return (c,f);
    }
}

final fn finalize(input: u32, f: Final) {
    f.run();
    assert_eq(input, 1u32);
}
