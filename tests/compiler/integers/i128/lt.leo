/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/lt.in
*/

transition main(a: i128, b: i128, c: bool) -> bool{
    return a < b == c;
}
