/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(public a: u8) -> u8 {
        async finalize(a);
        return a + a;
    }

    finalize foo(private a: u8) -> u8 {
        return a * a;
    }

    transition bar(public a: u8) -> u8 {
        async finalize (a);
        return a + a;
    }

    finalize bar(a: u8) -> private u8 {
        return a * a;
    }
}
