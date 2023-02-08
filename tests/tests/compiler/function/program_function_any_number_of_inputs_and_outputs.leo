/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition foo0_to_0() {}
    
    transition foo0_to_1() -> u8 {
        return 1u8 + 1u8;
    }
    
    transition foo1_to_0(a: u8) {}
    
    transition foo1_to_1(a: u8) -> u8 {
        return a * a;
    }
}
