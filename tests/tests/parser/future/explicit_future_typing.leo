/*
namespace: Parse
expectation: Pass
*/

import hello.aleo;
import bye.aleo;
program test.aleo {
    async transition main() -> Future<u32, u32, Future<()>, Future<u32, Future<u32, u32>>> {
        let f0: Future<()> = bye.aleo/add();
        let f1: Future<u32, Future<u32, u32>> = hello.aleo/nested_fn();
        let f2: Future<u32, u32, Future<()>, Future<u32, Future<u32, u32>>> = finalize_main(1u32, 1u32, f0, f1);
        return f2;
    }
    async function finalize_main(a: u32, b: u32, f0: Future<()>, f1: Future<u32, Future<u32, u32>>) {
        f0.await();
        f1.await();
        assert_eq(a,b);
    }
}
