/*
namespace: Parse
expectation: Fail
*/

stub foo.aleo {
    transition foo(x: u32, y: i32);
    finalize foo() {
        let a:u8 = 1u8;
    }
}

program test.aleo {
    transition doo(x: u32, y: i32) -> u32 {
        return 0u32;
    }
}