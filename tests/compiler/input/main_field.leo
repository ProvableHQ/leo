/*
namespace: Compile
expectation: Pass
input_file: inputs/main_field.in
*/

program test.aleo {    
    transition main(a: field, b: field, y: bool) -> bool {
        return y == true && a == b;
    }
}
