/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(a: i16, b: i16, c: i16) -> bool {
	    return a == 0x7Fi16 && b == 0x1Bi16 && c == 0b10101i16;
    }
}
