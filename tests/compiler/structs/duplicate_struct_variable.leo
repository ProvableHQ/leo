/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Bar {
        x: u32,
        x: u32,
    }
    
    function main() -> bool {
        return true;
    }
}
