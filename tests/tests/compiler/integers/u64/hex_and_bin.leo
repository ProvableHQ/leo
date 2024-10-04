/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(a: u64, b: u64, c: u64) -> bool {
	    return a == 0x7Fu64 && b == 0x1Bu64 && c == 0b10101u64;
    }
}
