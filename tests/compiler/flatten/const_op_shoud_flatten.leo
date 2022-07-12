/*
namespace: Compile
expectation: Pass
*/

function main () -> bool {
    let x: u8 = 2u8;
    x = x + 1u8;
    return x == 3u8;
}