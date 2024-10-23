/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(a: i128, b: i128, c: i128) -> bool {
	    return a == 0x7Fi128 && b == 0x1Bi128 && c == 0b10101i128;
    }
}
