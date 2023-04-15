/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    transition mint_public(public receiver: address, public amount: u64) {
        return then finalize(receiver, amount);
    }

    finalize mint_public (public receiver: address, public amount: u64) {
        let current_amount: u64 = account.get_or_init(receiver, 0u64);
        account.set(receiver, current_amount + amount);
    }

    transition finalize_no_params() {
        return then finalize();
    }

    finalize finalize_no_params() {
        let current_value: u8 = values.get_or_init(0u8, 0u8);
        values.set(0u8, current_value + 1u8);
        let current_amount: u64 = account.get_or_init(self.caller, 0u64);
        account.set(self.caller, current_amount + 1u64);
        current_amount = account.get(self.caller);
        account.set(self.caller, current_amount + 1u64);
    }
}