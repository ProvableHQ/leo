/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    function main(y: bool) -> bool {
        let (1u8+1u8,1u8+1u8): (u8,u8) = (1u8,2u8);
        return y;
    }
}