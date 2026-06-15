program test.aleo {
    record Token {
        owner: address,
        balance: field,
    }

    @noupgrade
    constructor() {}

    fn main(t: Token, x: u8) -> u8 {
        let unused_add: u8 = x.add_wrapped(1u8);
        let unused_div: u8 = x.div_wrapped(0u8);
        let unused_rem: u8 = x.rem_wrapped(0u8);
        let r: dyn record = t as dyn record;
        let unused_balance: field = r.balance;
        return x;
    }
}
