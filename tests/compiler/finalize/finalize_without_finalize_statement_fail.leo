/*
namespace: Compile
expectation: Fail
*/

mapping account: address => u64;

transition mint_public(public receiver: address, public amount: u64) {

}

finalize mint_public (public receiver: address, public amount: u64) {
    increment(account, receiver, amount);
}
