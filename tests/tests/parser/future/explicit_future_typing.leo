/*
namespace: Parse
expectation: Pass
*/
program test.aleo {
    async transition main() -> Future<Fn(u32, u32, Future<Fn()>, Future<Fn(u32, Future<Fn(u32, u32)>)>)> {
        let f0: Future<Fn()> = bye.aleo/add();
        let f1: Future<Fn(u32, Future<Fn(u32, u32)>)> = hello.aleo/nested_fn();
        let f2: Future<Fn(u32, u32, Future<Fn()>, Future<Fn(u32, Future<Fn(u32, u32)>)>)> = finalize_main(1u32, 1u32, f0, f1);
        return f2;
    }
    async function finalize_main(a: u32, b: u32, f0: Future<Fn()>, f1: Future<Fn(u32, Future<Fn(u32, u32)>)>) {
        f0.await();
        f1.await();
        assert_eq(a,b);
    }
}
