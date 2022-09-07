/*
namespace: Compile
expectation: Pass
*/

mapping account: address => u64;

@program
function mint_public(public receiver: address, public amount: u64) {
    finalize(receiver, amount);
} finalize (public receiver: address, public amount: u64) {
    increment(account, receiver, amount);
}
