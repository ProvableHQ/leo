
program test.aleo {
    transition main(a: u16, b: u16, c: u16) -> bool {
	    return a == 0x7Fu16 && b == 0x1Bu16 && c == 0b10101u16;
    }
}
