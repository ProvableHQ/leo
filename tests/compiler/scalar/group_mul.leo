/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalar_group.in
*/

function main(a: scalar, b: group, c: scalar) -> bool {
   let d: group = 1group * a;
   let e: group = a * 1group;
   let f: group = b * a;
   let g: group = a * b;

   return a * g == d;
}