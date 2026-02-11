program test.aleo {
    mapping account: address => u64;

    fn main() -> Final {
        return final { finalize(aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px, 200); };
    }


    @noupgrade
    constructor() {}
}

final fn finalize(public receiver: address, public amount: u64) {
    let current_amount: u64 = Mapping::get_or_use(account, receiver, 0u64);
    Mapping::set(account, receiver, current_amount + amount);
}

