/*
namespace: Compile
expectation: Pass
*/

mapping amounts: address => u128;

@program
function increase_self(amount: u128) {
    async finalize(self.caller, amount);
}

finalize increase_self(addr: address, amount: u128) {
    increment(amounts, addr, amount);
}
