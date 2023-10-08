/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    struct bar {
        data: u8,
    }

    transition foo(a: [bar; 8]) -> u8 {
        return a[0u8].data;
    }
}
