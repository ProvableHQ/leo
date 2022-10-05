/*
namespace: Compile
expectation: Pass
input_file: inputs/main.in
*/

program test.aleo {    
    transition main(a: bool) -> bool {
        return a == true;
    }
}
