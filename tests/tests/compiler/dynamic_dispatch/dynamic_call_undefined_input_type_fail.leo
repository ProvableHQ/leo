// FAIL: undefined type used as an input_types entry in _dynamic_call.
// `public Unknown` forces the parser to treat `Unknown` as a type so it lands in
// `input_types`. Type checking should emit ETYC0372017 without panicking.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u32) -> u32 {
        return _dynamic_call::[public Unknown, u32](target, net, func, x);
    }
}
