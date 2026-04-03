// FAIL: Intrinsic dynamic call passes concrete record instead of dyn record.
program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn main(prog: field, net: field, func: field, t: Token) -> dyn record {
        let result: dyn record = _dynamic_call::[Token, Token](prog, net, func, t);
        return result;
    }
}
