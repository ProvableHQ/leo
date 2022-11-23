/*
namespace: Parse
expectation: Pass
*/

program test.aleo {    
    function inf() -> u8 {
        inf();
    }
    
    function main(y: bool) -> bool {
        inf();
        return y;
    }
}
