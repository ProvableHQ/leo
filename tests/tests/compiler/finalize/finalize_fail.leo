/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    mapping values: u8 => u8;
    
    transition mint_public(public receiver: address, public amount: u64) {
        return then finalize(receiver, amount);
    }
    
    finalize mint_public (public receiver: address, public amount: u64) {
        increment(account, receiver, amount);
    }
    
    transition public_adder(public a: u8, public b: u8) {
        return then finalize(a, b);
    }

    finalize public_adder(a: u8, b: u8) -> public u8 {
        return a + b;
    }
    
    transition finalize_no_params() {
        return then finalize();
    }
    
    finalize finalize_no_params() {
        increment(values, 0u8, 1u8);
        increment(account, self.caller, 1u64);
    }
}
