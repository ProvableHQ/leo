/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> (bool) {
        return (a);
    }

    transition foo(a: bool, b: bool) -> (bool) {
        return (b,);
    }
}
