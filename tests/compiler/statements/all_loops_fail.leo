/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(k: bool) -> bool {
        let reverse: u32 = 0u32;
        for i: u32 in 9u32..0u32 {
            reverse = reverse + i;
        }
    
        let forward: u32 = 0u32;
        for x: u32 in 0u32..10u32 {
            forward = forward + x;
        }
    
        let reverse_inclusive: u32 = 0u32;
        for a: u32 in 10u32..=0u32 {
            reverse_inclusive = reverse_inclusive + a;
        }
    
        let forward_inclusive: u32 = 0u32;
        for b: u32 in 0u32..=10u32 {
            forward_inclusive = forward_inclusive + b;
        }
    
        return (reverse == forward) && (reverse_inclusive == forward_inclusive) && k;
    }
}
