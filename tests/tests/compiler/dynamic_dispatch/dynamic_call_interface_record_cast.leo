// OK: A router program accepts a dyn record and dispatches it via interface dynamic call.
// The router does not implement the interface — it acts purely as a dispatcher.
interface TokenOps {
    record Token;
    fn transfer(t: Token) -> Token;
}

program router.aleo {
    fn dispatch(target: field, token: dyn record) -> dyn record {
        return router.aleo::TokenOps@(target)::transfer(token);
    }
}
