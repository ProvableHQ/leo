/*
namespace: Parse
expectation: Fail
*/

stub credits.aleo {
    transition split(a1: credits, a2: u64) -> (credits, credits) {}

    function foo();
    finalize foo() {
        let c:u8 = 1u8;
    }
}

stub big.leo {
    transition split(a1: credits, a2: u64) -> (credits, credits) {
        credits.aleo/split(a1, a2);
    }
}

program test.aleo {
    transition foo(arg1: address, arg1: u64) {
        credits.aleo/transfer_public(arg1, arg2);
        credits.leo/transfer_public(arg1, arg2);
    }
}