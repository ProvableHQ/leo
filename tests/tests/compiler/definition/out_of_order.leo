/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    // @test
    // function fake_test() {}
    
    transition main(y: bool) -> bool {
        return y == true;
    }
    
    // struct Foo {}
}
