// FAIL: _dynamic_call cannot be used inside a final block.
program test.aleo {
    mapping result: u32 => u64;

    fn main(target: field, net: field, func: field) -> Final {
        return final {
            let val: u64 = _dynamic_call::[u64](target, net, func);
            Mapping::set(result, 0u32, val);
        };
    }
}
