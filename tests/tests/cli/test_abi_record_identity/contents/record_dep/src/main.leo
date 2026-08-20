program record_dep.aleo {
    record Token {
        owner: address,
        amount: u32,
    }

    @noupgrade
    constructor() {}
}
