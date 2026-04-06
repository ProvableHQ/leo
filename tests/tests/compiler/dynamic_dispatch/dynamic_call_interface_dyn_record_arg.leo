// OK: Dynamic call passing `dyn record` where the interface function expects a record type.
// The calling program does not implement the interface, so the record type is not in the
// program's symbol table. A `dyn record` argument should be accepted for any record-typed
// interface parameter.
interface ARC20 {
    record Token;
    fn transfer_private(token: Token, to: address) -> Token;
}

program foo.aleo {
    fn main(target: identifier, token: dyn record, to: address) -> dyn record {
        return ARC20@(target)::transfer_private(token, to);
    }
}
