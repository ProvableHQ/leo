const (HELLO,GOODBYE): (u8,u8) = (1u8, 1u8);

program test.aleo {
    fn foo(a: u8, b: u8, flag: bool) -> u8 {
        return GOODBYE;
    }
}
