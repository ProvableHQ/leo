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
        let current_amount: u64 = Mapping::get_or_init(account, receiver, 0u64);
        Mapping::set(account, receiver, current_amount + amount);
    }

    transition finalize_no_params() -> () {
        return then finalize();
    }

    finalize finalize_no_params() -> () {
        let current_value: u8 = Mapping::get_or_init(values, 0u8, 0u8);
        Mapping::set(values, 0u8, current_value + 1u8);
        let current_amount: u64 = Mapping::get_or_init(account, self.caller, 0u64);
        Mapping::set(account, self.caller, current_amount + 1u64);
    }
}