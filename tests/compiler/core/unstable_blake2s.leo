// namespace: Compile
// expectation: Pass
// inputs:
//  - blake.in: |
//     [main]
//     message: [u8; 32] = [0; 32];

//     [registers]
//     r0: [u8; 32] = [0; 32];

import core.unstable.blake2s.Blake2s;

function main(message: [u8; 32]) -> [u8; 32] {
    const seed: [u8; 32] = [1; 32];
    let result = Blake2s::hash(seed, message);

    console.log("Result: {}", result);

    return result;
}
