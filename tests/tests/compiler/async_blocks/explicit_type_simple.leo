program test.aleo {
    mapping foo: u32 => u32;
    async transition main_inner(public a: u32, b: u32) -> (u32, Future<Fn(u32)>) {
        let c: u32 = a + b;
        let x = 1u32;
        let f: Future<Fn(u32)> = async { // captures only `x`
            Mapping::set(foo, a, 1u32);
        };
        return (c, f);
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    async transition main(public a: u32, b: u32) -> (u32, Future<Fn(Future<Fn(u32)>, u32, u32)>) {
        let c: u32 = a + b;
        let (d, f1): (u32, Future<Fn(u32)>) = test.aleo/main_inner(1u32, 1u32);
        let f:Future<Fn(Future<Fn(u32)>, u32, u32)> = async { // captures `f1`, `a`, and `c` in this order
            f1.await();
            assert_eq(a, c);
        }; 
        return (c,f);
    }
}
