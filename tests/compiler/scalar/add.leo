/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalars.in
*/

transition main(a: scalar, b: scalar, c: scalar) -> bool {
   return a + b == c;
}