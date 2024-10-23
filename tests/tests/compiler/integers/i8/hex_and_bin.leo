/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(a: i8, b: i8, c: i8) -> bool {
	    return a == 0x7Fi8 && b == 0x1Bi8 && c == 0b10101i8;
    }
}
