/*
namespace: Compile
expectation: Fail
*/


transition mint_public(public receiver: address, public amount: u64) {
    async finalize(receiver, amount);
}

finalize mint_public (public receiver: address, public amount: u64) {}
