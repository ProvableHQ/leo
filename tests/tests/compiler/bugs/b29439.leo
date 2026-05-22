// Regression test for https://github.com/ProvableHQ/leo/issues/29439.
// A record that declares `owner` after a non-empty field, passed through a
// function unchanged. The codegen `clone_register` Type::Composite arm must
// emit the cast operand for `owner` first to match the declaration emitter
// in `visit_record`; otherwise snarkVM rejects the bytecode with
// `ECLI0377047: Leo generated invalid Aleo bytecode`
// (`Casting to a record requires the first operand to be an address`).
program test.aleo {
    @noupgrade
    constructor() {}

    record Foo {
        x: u32,
        owner: address,
    }

    fn pass(r: Foo) -> Foo {
        return r;
    }
}
