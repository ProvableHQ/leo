/*
namespace: Compile
expectation: Fail
*/

stub credits.aleo {
    record credits {
        owner: address,
        micrcredits: u64,
    }

    transition mint(public a1: address, public a2: u64) -> credits {
    } finalize mint() {
        let a:u8 = 1u8;
    }
}

program test.aleo {
    transition foo(arg1: address, arg2: u64) {
        let a:u8 = 1u8;
    }
}