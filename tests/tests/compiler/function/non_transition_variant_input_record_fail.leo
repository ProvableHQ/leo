/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record credits {
        owner: address,
        micrcredits: u64,
    }

    function foo(a: credits) -> u8 {
        return 1u8;
    }

    function boo(a: address, b: u64) -> credits {
        return credits {
           owner: a,
           micrcredits: b,
       };
    }
}
