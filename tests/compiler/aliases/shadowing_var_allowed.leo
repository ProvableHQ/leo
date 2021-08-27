/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/dummy.in
*/

type int = u32;

function main(y: bool) -> bool {
    let int: int = 1u32;
    
    return y;
}