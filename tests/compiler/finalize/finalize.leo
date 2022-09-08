/*
namespace: Compile
expectation: Pass
*/

mapping account: address => u64;
mapping values: u8 => u8;

@program
function mint_public(public receiver: address, public amount: u64) {
    finalize(receiver, amount);
} finalize (public receiver: address, public amount: u64) {
    increment(account, receiver, amount);
}

@program
function public_adder(public a: u8, public b: u8) {
    finalize(a, b);
} finalize(a: u8, b: u8) -> public u8 {
    return a + b;
}

@program
function finalize_no_params() {
    finalize();
} finalize() {
    increment(values, 0u8, 1u8);
    increment(account, self.caller, 1u64);
}
