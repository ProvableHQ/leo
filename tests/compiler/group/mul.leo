/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalar_group.in
*/

function main(a: scalar, b: group) -> group {
   return a * b;
}