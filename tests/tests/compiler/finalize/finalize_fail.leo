/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    mapping values: u8 => u8;
    
    async transition mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount);
    }
    
    async function finalize_mint_public(public receiver: address, public amount: u64) {
        increment(account, receiver, amount);
    }
    
    async transition public_adder(public a: u8, public b: u8) -> Future {
        return finalize_public_adder(a, b);
    }

    async function finalize_public_adder(a: u8, b: u8) -> public u8 {
        return a + b;
    }
    
    async transition finalize_no_params() -> Future {
        return finalize_no_params();
    }
    
    async function finalize_no_params() {
        increment(values, 0u8, 1u8);
        increment(account, self.caller, 1u64);
    }
}
