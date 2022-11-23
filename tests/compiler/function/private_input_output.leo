/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(private a: u8, public b: u8) -> private u8 {
        let c: u8 = a + b;
        return c;
    }
}
