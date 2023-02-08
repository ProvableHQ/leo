/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: i8) -> bool {
        let b: bool = a == 1u8;
        let c: bool = a != 1u8;
        let d: bool = a > 1u8;
        let e: bool = a < 1u8;
        let f: bool = a >= 1u8;
        let g: bool = a <= 1u8;
        let h: u32 = a * 1u8;
        return b;
    }}
