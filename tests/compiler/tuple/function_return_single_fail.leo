/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/bool_bool.in
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> (bool) {
        return (a);
    }

    transition foo(a: bool, b: bool) -> (bool) {
        return (b,);
    }
}
