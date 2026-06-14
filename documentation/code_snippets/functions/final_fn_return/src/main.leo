// ANCHOR: file
// A `final fn` may declare an output type and return a value.
final fn capped_amount(receiver: address, amount: u64) -> u64 {
    let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
    if current + amount > 1000u64 {
        return 1000u64;
    }
    return current + amount;
}

program final_fn_return_demo.aleo {
    mapping account: address => u64;

    fn deposit(public receiver: address, public amount: u64) -> Final {
        return final {
            // The returned value is bound at the call site.
            let credited: u64 = capped_amount(receiver, amount);
            Mapping::set(account, receiver, credited);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
