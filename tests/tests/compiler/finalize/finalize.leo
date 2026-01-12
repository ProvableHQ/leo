
program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint(receiver, amount); };
    }

    fn finalize_self_caller() -> Final {
        let caller = self.caller;
        return final { finalize_finalize_self_caller(caller); };
    }
}

final fn finalize_mint(public receiver: address, public amount: u64) {
    let current_amount: u64 = Mapping::get_or_use(account, receiver, 0u64);
    Mapping::set(account, receiver, current_amount + amount);
}

final fn finalize_finalize_self_caller(caller: address) -> () {
    let current_value: u8 = Mapping::get_or_use(values, 0u8, 0u8);
    Mapping::set(values, 0u8, current_value + 1u8);
    let current_amount: u64 = Mapping::get_or_use(account, caller, 0u64);
    Mapping::set(account, caller, current_amount + 1u64);
}
