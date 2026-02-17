
program test.aleo {
    mapping amounts: address => u128;

    fn decrease_self(amount: u128) -> Final {
        return final { finalize_decrease_self(self.caller, amount); };
    }
}

final fn finalize_decrease_self(addr: address, amount: u128) {
    let result: () = Mapping::set(amounts, addr, amount);
    let result: u128 = Mapping::set(amounts, addr, amount);
}
