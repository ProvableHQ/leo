/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(a: u128, b: u128, c: u128) -> bool {
	    return a == 0x7Fu128 && b == 0x1Bu128 && c == 0b10101u128;
    }
}
