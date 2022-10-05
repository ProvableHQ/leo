/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

program test.aleo {    
    // @test
    // function fake_test() {}
    
    transition main(y: bool) -> bool {
        return y == true;
    }
    
    // struct Foo {}
}
