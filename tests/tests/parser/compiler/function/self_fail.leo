/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition matches(addr: address) -> bool {
        return self.foo == addr;
    }
}
