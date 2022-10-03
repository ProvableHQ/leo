/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalar_group.in
*/

transition main(a: scalar, b: group) -> group {
   return a * b;
}