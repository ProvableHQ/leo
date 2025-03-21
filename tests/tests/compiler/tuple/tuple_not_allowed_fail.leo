
program test.aleo {
    function main(zz: bool) -> u8 {
        return 1u8;
    }

    function foo(a: (u8, u16)) -> (u8, u16) {
        return (1u8, 2u16);
    }

    function bar(zz: bool) -> (u8, (u16, u32)) {
        return (1u8, (2u16, 3u32));
    }

    function baz(zz: bool) -> u8 {
        for i: (u8, u16) in 0u8..2u8 {}
        return 0u8;
    }

    struct A {
        mem: (u8, u16)
    }
}
