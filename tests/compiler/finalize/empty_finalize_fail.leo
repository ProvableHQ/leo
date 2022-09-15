/*
namespace: Compile
expectation: Fail
*/


@program
function mint_public(public receiver: address, public amount: u64) {
    finalize(receiver, amount);
}

finalize mint_public (public receiver: address, public amount: u64) {}
