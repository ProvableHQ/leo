/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/basic.in
*/

type x = u32;

function main(x: x, y: bool) -> bool {
    return y;
}