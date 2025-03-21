
program test.aleo {
    transition main(a: u8, b: u8, c: u8) -> bool {
	    return a == 0x7Fu8 && b == 0x1Bu8 && c == 0b10101u8;
    }
}
