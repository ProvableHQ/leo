/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(a: i32, b: i32, c: i32) -> bool {
	    return a == 0x7Fi32 && b == 0x1Bi32 && c == 0b10101i32;
    }
}
