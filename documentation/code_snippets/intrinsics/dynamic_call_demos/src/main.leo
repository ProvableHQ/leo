program dynamic_call_demos.aleo {
    fn dc_void(prog: identifier, net: identifier, func: identifier, x: u64) {
        // ANCHOR: dc_void
        _dynamic_call(prog, net, func, x);
        // ANCHOR_END: dc_void
    }

    fn dc_return(prog: identifier, net: identifier, func: identifier, x: u64) -> u64 {
        // ANCHOR: dc_return
        let result: u64 = _dynamic_call::[u64](prog, net, func, x);
        // ANCHOR_END: dc_return
        return result;
    }

    fn dc_public_input(prog: identifier, net: identifier, func: identifier, x: u64) -> u64 {
        // ANCHOR: dc_public_input
        // One public input, then the return type
        let result: u64 = _dynamic_call::[public u64, u64](prog, net, func, x);
        // ANCHOR_END: dc_public_input
        return result;
    }

    fn dc_multiple_inputs(prog: identifier, net: identifier, func: identifier, x: u32, y: u32) -> (u32, u32) {
        // ANCHOR: dc_multiple_inputs
        let (a, b): (u32, u32) = _dynamic_call::[public u32, public u32, (u32, u32)](prog, net, func, x, y);
        // ANCHOR_END: dc_multiple_inputs
        return (a, b);
    }

    fn dc_void_with_annotations(prog: identifier, net: identifier, func: identifier, x: u64) {
        // ANCHOR: dc_void_annotated
        _dynamic_call::[public u64, ()](prog, net, func, x);
        // ANCHOR_END: dc_void_annotated
    }

    fn dc_final_return(target: identifier, val: u32) -> (u32, Final) {
        // ANCHOR: dc_final
        let (v, f): (u32, Final) = _dynamic_call::[(u32, Final)](target, 'aleo', 'increment', val);
        return (v, final {
            f.run();
        });
        // ANCHOR_END: dc_final
    }

    fn dc_dyn_record(prog: identifier, net: identifier, func: identifier, token: dyn record) -> dyn record {
        // ANCHOR: dc_dyn_record
        let result: dyn record = _dynamic_call::[dyn record, dyn record](prog, net, func, token);
        // ANCHOR_END: dc_dyn_record
        return result;
    }

    fn final_reads(prog: field, net: field, map: field, key: address) -> Final {
        return final {
            // ANCHOR: dyn_contains
            let exists: bool = _dynamic_contains(prog, net, map, key);
            // ANCHOR_END: dyn_contains
            let _ = exists;
        };
    }

    fn final_get(prog: field, net: field, map: field, key: address) -> Final {
        return final {
            // ANCHOR: dyn_get
            let val: u64 = _dynamic_get::[u64](prog, net, map, key);
            // ANCHOR_END: dyn_get
            let _ = val;
        };
    }

    fn final_get_literal(key: address) -> Final {
        return final {
            // ANCHOR: dyn_get_literal
            let val: u64 = _dynamic_get::[u64]('some_program', 'aleo', 'balances', key);
            // ANCHOR_END: dyn_get_literal
            let _ = val;
        };
    }

    fn final_get_or_use(prog: field, net: field, map: field, key: address, fallback: u64) -> Final {
        return final {
            // ANCHOR: dyn_get_or_use
            let val: u64 = _dynamic_get_or_use::[u64](prog, net, map, key, fallback);
            // ANCHOR_END: dyn_get_or_use
            let _ = val;
        };
    }

    @noupgrade
    constructor() {}
}
