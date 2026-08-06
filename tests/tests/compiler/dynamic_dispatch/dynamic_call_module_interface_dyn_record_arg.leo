// --- library: ops_lib --- //

// --- Next Module: algorithms.leo --- //

export interface Processor {
    record Token;
    fn process(token: Token) -> Token;
}

// --- Next Program --- //

program foo.aleo {
    fn main(target: identifier, token: dyn record) -> dyn record {
        return ops_lib::algorithms::Processor@(target)::process(token);
    }

    @noupgrade
    constructor() {}
}
