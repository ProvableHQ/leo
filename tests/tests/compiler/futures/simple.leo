/*
namespace: Compile
expectation: Pass
*/
program test.aleo {
    mapping foo: u32 => u32;
    async transition main_inner(public a: u32, b: u32) -> (u32, Future) {
        let c: u32 = a + b;
        let f: Future = finalize();
        return (c, f);
    }

    async function finalize() {
        Mapping::set(foo, 1u32, 1u32);
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    async transition main(public a: u32, b: u32) -> (u32, Future) {
        let c: u32 = a + b;
        let (d, f1): (u32, Future) = test.aleo/main_inner(1u32, 1u32);
        let f:Future = finalize(c, f1);
        return (c,f);
    }

    async function finalize(input: u32, f: Future) {
        f.await();
        assert_eq(input, 1u32);
    }
}