/*
namespace: Compile
expectation: Pass
*/

mapping amounts: address => u128;

@program
function decrease_self(amount: u128) {
    finalize(self.caller, amount);
} finalize(addr: address, amount: u128) {
    decrement(amounts, addr, amount);
}
