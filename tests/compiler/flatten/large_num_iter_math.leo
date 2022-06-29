/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(b: bool) -> bool {
    for i: u128 in 100430379905342355364592083543776445547u128..100430379905342355364592083543776445553u128 {
        let x: u128 = 94673869619048365753799855214640434430u128 / i;
        let y: u128 = i - x;
    }
    return true == b;
}