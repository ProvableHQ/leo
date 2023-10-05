/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    function main(y: bool) -> bool {
        let (a,b,c): (u8,u8) = (2u8,3u8);
        let (d,e): (u8,u8,u8) = (1u8,2u8,3u8);
        let (g,h,i): (u8,u8,u8) = (1u8);
        return y;
    }
}

