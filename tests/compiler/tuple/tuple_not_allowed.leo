/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() -> u8 {
        return 1u8;
    }
    
    function foo(a: (u8, u16)) -> (u8, u16) {
        return (1u8, 2u16);
    }
    
    function bar() -> (u8, (u16, u32)) {
        return (1u8, (2u16, 3u32));
    }
    
    function baz() -> u8 {
        for i: (u8, u16) in 0u8..2u8 {}
        return 0u8;
    }
    
    struct A {
        mem: (u8, u16)
    }
}
