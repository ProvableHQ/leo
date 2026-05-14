// snarkVM PR #3253 relaxes the view-body parser to `many0`, so a `view fn` whose
// body emits zero commands (no body, identity, or just a literal return) compiles
// to a valid Aleo view without any dummy filler.
program test.aleo {
    fn main(x: u32) -> u32 {
        return x;
    }

    // No body, no output (returns Unit).
    view fn empty() {}

    // No body, output forwards the input register directly.
    view fn echo(x: u64) -> u64 {
        return x;
    }

    // No body, output is a literal.
    view fn pi() -> u64 {
        return 314u64;
    }

    @noupgrade
    constructor() {}
}
