program token_a.aleo {
    fn transfer(to: address, amount: u64) {
        let _ = (to, amount);
        return;
    }

    @noupgrade
    constructor() {}
}
