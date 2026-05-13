// ANCHOR: file
interface Transfer {
    record Token;
    fn transfer(input: Token, to: address, amount: u64) -> Token;
}
// ANCHOR_END: file

program decl_transfer.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}
