/*
namespace: Compile
expectation: Pass
*/

record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

function mint(r0: address, r1: u64) -> Token {
    return Token {
        owner: r0,
        gates: 0u64,
        amount: r1,
    };
}

function main(x: address) -> u64 {
    const c: u64 = 1u64;
    let t: Token = Token { owner: x, gates: 0u64, amount: c};

    return t.gates;
}