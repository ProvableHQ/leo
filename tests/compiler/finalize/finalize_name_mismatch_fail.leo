/*
namespace: Compile
expectation: Fail
*/

mapping account: address => u64;
mapping values: u8 => u8;

@program
function mint_public(public receiver: address, public amount: u64) {
    finalize(receiver, amount);
}

finalize mint_private (public receiver: address, public amount: u64) {
    increment(account, receiver, amount);
}

