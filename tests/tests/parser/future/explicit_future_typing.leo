program test.aleo {
    fn main() -> Final<Fn(u32, u32, Final<Fn()>, Final<Fn(u32, Final<Fn(u32, u32)>)>)> {
        let f0: Final<Fn()> = bye.aleo/add();
        let f1: Final<Fn(u32, Final<Fn(u32, u32)>)> = hello.aleo/nested_fn();
        let f2: Final<Fn(u32, u32, Final<Fn()>, Final<Fn(u32, Final<Fn(u32, u32)>)>)> = final { finalize_main(1u32, 1u32, f0, f1); };
        return f2;
    }
}
final fn finalize_main(a: u32, b: u32, f0: Final<Fn()>, f1: Final<Fn(u32, Final<Fn(u32, u32)>)>) {
    f0.run();
    f1.run();
    assert_eq(a,b);
}
