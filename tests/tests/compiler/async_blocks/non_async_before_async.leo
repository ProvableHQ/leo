program test.aleo {
    mapping foo: u32 => u32;

    fn main_inner(public a: u32, public b: u32) -> Final {
        return final { Mapping::set(foo, a, b); };
    }

    fn baz(a: u32, b: u32) -> u32 {
        return a + b;
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    fn main(public a: u32, b: u32) -> Final {
        let sum1: u32 = test.aleo/baz(a, b);
        assert_eq(sum1, 1u32);

        let f1: Final = test.aleo/main_inner(0u32, 0u32);

        let sum2: u32 = test.aleo/baz(a, b);
        assert_eq(sum2, 1u32);

        let f2: Final = test.aleo/main_inner(1u32, 1u32);

        let sum3: u32 = test.aleo/baz(a, b);
        assert_eq(sum3, 1u32);

        let f:Final = final { f1.run(); f2.run(); };
        return f;
    }
}
