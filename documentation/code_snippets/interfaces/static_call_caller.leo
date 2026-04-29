program token_a.aleo {
    fn transfer(to: address, amount: u64) {
        let _ = (to, amount);
        return;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import token_a.aleo;

program static_caller.aleo {
    // ANCHOR: snippet
    // Static: the callee is fixed at compile time
    fn route_transfer_static(to: address, amount: u64) {
        return token_a.aleo::transfer(to, amount);
    }
    // ANCHOR_END: snippet

    @noupgrade
    constructor() {}
}
