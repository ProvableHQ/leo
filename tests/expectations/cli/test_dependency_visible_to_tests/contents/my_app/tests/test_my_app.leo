import my_app.aleo;

program test_my_app.aleo {
    @noupgrade
    constructor() {}

    // `mathlib` is only in `dependencies`, not `dev_dependencies`, yet the test program must
    // still be able to name its types. Regression test for #29592.
    @test
    fn test_combine() {
        let p: mathlib::Pair = mathlib::Pair { a: 3u32, b: 4u32 };
        assert_eq(my_app.aleo::combine(p.a, p.b), 7u32);
    }
}
