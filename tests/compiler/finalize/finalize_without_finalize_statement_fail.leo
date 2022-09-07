/*
namespace: Compile
expectation: Fail
*/

mapping account: address => u64;

@program
function mint_public(public receiver: address, public amount: u64) {

} finalize (public receiver: address, public amount: u64) {
    increment(account, receiver, amount);
}
