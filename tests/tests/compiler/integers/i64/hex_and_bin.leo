/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(a: i64, b: i64, c: i64) -> bool {
	    return a == 0x7Fi64 && b == 0x1Bi64 && c == 0b10101i64;
    }
}
