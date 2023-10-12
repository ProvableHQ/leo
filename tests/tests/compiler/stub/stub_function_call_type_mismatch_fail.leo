/*
namespace: Compile
expectation: Fail
*/

stub credits.aleo {
    record credits {
        owner: address,
        micrcredits: u64,
    }

    transition mint(public a1: address, public a2: u64) -> credits;
}

program test.aleo {
    transition foo(arg1: address, arg2: u64) {
        // External call
        credits.aleo/mint(arg1, 1u32);
    }
}