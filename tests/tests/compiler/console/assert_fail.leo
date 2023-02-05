/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    transition main(a: bool) -> bool {
        assert_eq(a == 1u8);
        assert(1u8);
    
        return a == true;
    }
}
