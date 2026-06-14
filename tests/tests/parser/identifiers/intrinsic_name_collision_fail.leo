// A local binding whose name matches a bare-callable intrinsic would be silently
// shadowed by the intrinsic at every call site (parser dispatches `_self_caller()`
// to the intrinsic before any scope is consulted). Reject the binding here.

program test.aleo {
    fn main() -> address {
        let _self_caller: address = aleo1qqq00000000000000000000000000000000000000000000000000004hkwch;
        return _self_caller;
    }
}
