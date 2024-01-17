/*
namespace: Parse
expectation: Fail
*/

stub credits.aleo {
    transition split(a1: credits, a2: u64) -> (credits, credits) {}
}

program test.aleo {
    import hello.aleo;
    transition foo(arg1: address, arg1: u64) {
        credits.aleo/transfer_public(arg1, arg2);
        credits.leo/transfer_public(arg1, arg2);
    }
}