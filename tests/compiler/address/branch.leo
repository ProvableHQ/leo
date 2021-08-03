/*
namespace: Compile
expectation: Pass
input_file: inputs/branch.in
*/


function main (x: address, y: bool) -> bool {
    let z = aleo18cw5zdez3zhypev3tnfhmwvhre9ramwle4up947gcyy5rnmjw5yqn93wsr;
    if y {
        z = aleo1f2gs8g0qpumlgzpvmkw3q07y6xrwsdr0lqsu9h9fgnh8d7e44v9qhpgpkj;
    }

    return z == aleo1f2gs8g0qpumlgzpvmkw3q07y6xrwsdr0lqsu9h9fgnh8d7e44v9qhpgpkj;
}