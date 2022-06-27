/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let a: group = Pedersen64::hash(1u128);

    return true;
}