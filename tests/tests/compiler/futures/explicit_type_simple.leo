program test.aleo {
    mapping foo: u32 => u32;
    fn main_inner(public a: u32, b: u32) -> (u32, Final<Fn()>) {
        let c: u32 = a + b;
        let f: Final<Fn()> = final { finalize(1u32); };
        return (c, f);
    }
}

final fn finalize(a:u32) {
    Mapping::set(foo, a, 1u32);
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final<Fn(u32, u32, Final<Fn()>)>) {
        let c: u32 = a + b;
        let (d, f1): (u32, Final<Fn()>) = test.aleo/main_inner(1u32, 1u32);
        let f:Final<Fn(u32, u32, Final<Fn()>)> = final { finalize(a, c, f1); };
        return (c,f);
    }
}

final fn finalize(a: u32, b:u32, f: Final<Fn()>) {
    f.run();
    assert_eq(a, b);
}
