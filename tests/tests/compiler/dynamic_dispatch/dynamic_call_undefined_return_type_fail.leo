// FAIL: undefined type in the return_types of _dynamic_call, no final block.
// `(u32, Unknown)` is a tuple whose first element is a primitive, so the parser
// treats it as a type and `Unknown` lands in return_types.
// Verifies that check_dynamic_call emits ETYC0372017 directly, independent of
// the visit_async panic path fixed in issue #29306.
program test.aleo {
    fn main(target: field, net: field, func: field) -> (u32, u32) {
        return _dynamic_call::[(u32, Unknown)](target, net, func);
    }
}
