/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    mapping bar: u8 => u8;
    
    transition bar(a: u8) -> u8 {
        return a + 1u8;
    }
    
    transition foo(bar: u8) -> u8 {
        return bar + 1u8;
    }
    
    transition baz(a: u8) -> u8 {
        let bar: u8 = a + 1u8;
        return bar;
    }
    
    struct bar {
        a: u64;
    }
}
