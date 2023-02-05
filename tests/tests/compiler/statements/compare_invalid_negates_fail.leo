/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: u8) -> bool {
        let b: bool = -a == -1u8;
        let c: bool = -a > -1u8;
        let d: bool = -a < -1u8;
        let e: bool = -a >= -1u8;
        let f: bool = -a <= -1u8;
        let g: u8 = -a * -1u8;
        let h: u8 = -a ** -1u8;
        return b;
    }}
