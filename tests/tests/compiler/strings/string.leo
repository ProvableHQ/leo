/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b * 1u32; // c = 3
        let str:string = "a a a";
        if(str == "b b  b") {
            c = 123u32;
        } else {
            c = 321u32;
        }
        return c;
    }}
