/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/bool_bool.in
*/

program test.aleo {
    transition main(a: bool, b: bool) -> () {
        return;
    }
}
