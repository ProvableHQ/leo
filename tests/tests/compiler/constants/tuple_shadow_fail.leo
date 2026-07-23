export const (HELLO,GOODBYE): (u8,u8) = (0u8,0u8);
export const HELLO: u8 = 0u8;

program test.aleo {
    fn foo(a: u8, b: u8, flag: bool) -> u8 {
        return 1u8;
    }

    @noupgrade
    constructor() {}
}
