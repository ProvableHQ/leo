// OK: Intrinsic dynamic call casts record to dyn record.
program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn main(prog: field, net: field, func: field, t: Token) -> dyn record {
        let result: dyn record = _dynamic_call::[dyn record, dyn record](prog, net, func, t as dyn record);
        return result;
    }
}
