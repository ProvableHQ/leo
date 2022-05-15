/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalars.in
*/

function main(a: scalar, b: scalar) -> bool {
   return -a == -b;
}