/*
namespace: Compile
expectation: Fail
*/

mapping account: address => u64;

@program
function mint_public(public receiver: address, public amount: u64) {
    finalize(receiver, amount);
}

finalize mint_public (public receiver: address, constant amount: u64) -> constant u64 {
    increment(account, receiver, amount);
}

@program
function mint_public2(public receiver: address, public amount: u64) {
    finalize(receiver, amount);
}

finalize mint_public2(public receiver: address, amount: u64) -> u64 {
    increment(account, receiver, amount);
    return amount + amount;
}
