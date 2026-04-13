// FAIL: undefined types in both input_types and return_types of _dynamic_call.
// `public BadInput` forces type parsing so it lands in input_types.
// `(u32, BadReturn)` is a tuple parsed as a type; BadReturn is unpacked into return_types.
// Both ETYC0372017 diagnostics should be reported.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u32) -> (u32, u32) {
        return _dynamic_call::[public BadInput, (u32, BadReturn)](target, net, func, x);
    }
}
