/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(a: u32, b: u32, c: u32) -> bool {
	    return a == 0x7Fu32 && b == 0x1Bu32 && c == 0b10101u32;
    }
}
