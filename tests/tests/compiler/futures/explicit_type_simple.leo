/*
namespace: Compile
expectation: Pass
*/
program test.aleo {
    mapping foo: u32 => u32;
    async transition main_inner(public a: u32, b: u32) -> (u32, Future<Fn(u32)>) {
        let c: u32 = a + b;
        let f: Future<Fn(u32)> = finalize(1u32);
        return (c, f);
    }

    async function finalize(a:u32) {
        Mapping::set(foo, a, 1u32);
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    async transition main(public a: u32, b: u32) -> (u32, Future<Fn(u32, u32, Future<Fn(u32)>)>) {
        let c: u32 = a + b;
        let (d, f1): (u32, Future<Fn(u32)>) = test.aleo/main_inner(1u32, 1u32);
        let f:Future<Fn(u32, u32, Future<Fn(u32)>)> = finalize(a, c, f1);
        return (c,f);
    }

    async function finalize(a: u32, b:u32, f: Future<Fn(u32)>) {
        f.await();
        assert_eq(a, b);
    }
}