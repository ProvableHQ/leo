/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record bar {
        owner: address,
        data: u8,
    }

    transition foo(a: [bar; 8]) -> u8 {
        return a[0u32].data;
    }
}
