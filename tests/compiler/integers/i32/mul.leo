/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/mul.in
    
*/

transition main(a: i32, b: i32, c: i32) -> bool {
    return a * b == c;
}
