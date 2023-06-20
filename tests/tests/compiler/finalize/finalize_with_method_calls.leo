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
        let current_amount: u64 = account.get_or_use(receiver, 0u64);
        account.set(receiver, current_amount + amount);
    }

    transition finalize_self_caller() {
        return then finalize(self.caller);
    }

    finalize finalize_self_caller(caller: address) {
        let current_value: u8 = values.get_or_use(0u8, 0u8);
        values.set(0u8, current_value + 1u8);
        let current_amount: u64 = account.get_or_use(caller, 0u64);
        account.set(caller, current_amount + 1u64);
        current_amount = account.get(caller);
        account.set(caller, current_amount + 1u64);
    }
}