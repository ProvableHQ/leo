program test.aleo {
    mapping foo: u32 => u32;
    fn main_inner(public a: u32, b: u32) -> (u32, Final<Fn(u32)>) {
        let c: u32 = a + b;
        let x = 1u32;
        let f: Final<Fn(u32)> = final { // captures only `x`
            Mapping::set(foo, a, 1u32);
        };
        return (c, f);
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final<Fn(Final<Fn(u32)>, u32, u32)>) {
        let c: u32 = a + b;
        let (d, f1): (u32, Final<Fn(u32)>) = test.aleo/main_inner(1u32, 1u32);
        let f:Final<Fn(Final<Fn(u32)>, u32, u32)> = final { // captures `f1`, `a`, and `c` in this order
            f1.run();
            assert_eq(a, c);
        }; 
        return (c,f);
    }
}
