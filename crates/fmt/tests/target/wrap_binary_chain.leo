program test.aleo {
    fn compute(f1: Foo, f2: Foo, i: u64) -> u64 {
        let sum: u64 = library.aleo/X
            + library.aleo/foo()
            + library.aleo/foo()
            + bar(i)
            + library.aleo/boo()
            + library.aleo/boo2()
            + f1.x
            + f2.x;
        return sum;
    }
}
