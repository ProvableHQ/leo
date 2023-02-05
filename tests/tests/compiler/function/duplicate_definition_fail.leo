/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(y: bool) -> bool {
        console.log("{}", 1u8);
        return y;
    }
    
    function main(y: bool) -> bool {
        console.log("{}", 2u8);
        return y; 
    }
}
