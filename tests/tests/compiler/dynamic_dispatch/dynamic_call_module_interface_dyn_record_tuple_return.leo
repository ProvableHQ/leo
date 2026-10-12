// --- library: ops_lib --- //

// --- Next Module: algorithms.leo --- //

export interface Processor {
    record Token;
    fn process(token: Token) -> (Token, u64);
}

// --- Next Program --- //

program foo.aleo {
    fn main(target: identifier, token: dyn record) -> (dyn record, u64) {
        return ops_lib::algorithms::Processor@(target)::process(token);
    }

    @noupgrade
    constructor() {}
}
