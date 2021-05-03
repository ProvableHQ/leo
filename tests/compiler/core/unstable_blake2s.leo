/*
namespace: Compile
expectation: Pass
inputs:
 - blake.in: |
    [constants]
    message: [u8; 32] = [0; 32];

    [registers]
    r0: [u8; 32] = [0; 32];
*/

import core.unstable.blake2s.Blake2s;

function main(const message: [u8; 32]) -> [u8; 32] {
    const seed: [u8; 32] = [1; 32];
    const result = Blake2s::hash(seed, message);

    console.log("Result: {}", result);

    return result;
}
