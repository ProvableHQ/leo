/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    record bar {
        owner: address,
        data: u8,
    }

    transition foo(a: [bar; 8]) -> u8 {
        return a[0].data;
    }
}
