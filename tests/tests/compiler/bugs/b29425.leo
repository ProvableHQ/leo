// Regression test for https://github.com/ProvableHQ/leo/issues/29425.
// The codegen `clone_register` Type::Composite arm previously emitted a
// MemberAccess operand for every member, including empty-typed members
// (Type::Unit or zero-length Type::Array). The record declaration emitter
// omits empty-typed fields, so the cast operand list disagreed with the
// declared schema and snarkVM rejected the bytecode with
// `ECLI0377047: Leo generated invalid Aleo bytecode`.
program bugpoc.aleo {
    @noupgrade
    constructor() {}

    record Token {
        owner: address,
        blob: [u64; 0],
    }

    fn pass(t: Token) -> Token {
        return t;
    }
}
