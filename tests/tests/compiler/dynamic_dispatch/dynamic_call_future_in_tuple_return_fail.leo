// FAIL: Returning a dynamic future bare inside a tuple — must be consumed by a final block.
program test.aleo {
    fn main(prog: field, net: field, func: field, arg: u64) -> (u64, Final) {
        let f: Final = _dynamic_call::[Final](prog, net, func, arg);
        return (42u64, f);
    }
}
