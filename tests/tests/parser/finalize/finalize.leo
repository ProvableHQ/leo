/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    function main() -> bool {
    
    }
    
    finalize main() {
    
    }
    
    function main() -> bool {
    
    } finalize main(a: foo, b: bar) -> baz {
    
    }
    
    
}
