// _dynamic_call returning a Future, consumed in a final block.
program test.aleo {
    mapping counts: u32 => u32;

    fn main(target: field, net: field, func: field, x: u32) -> (u32, Final) {
        let (val, f): (u32, Final) = _dynamic_call::[(u32, Final)](target, net, func, x);
        return (val, final { finalize_main(f); });
    }
}

final fn finalize_main(f: Final) {
    f.run();
}
