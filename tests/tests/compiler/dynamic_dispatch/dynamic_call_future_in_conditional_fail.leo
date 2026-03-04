program test.aleo {
    fn main(cond: bool, prog: field, net: field, func: field) -> Final {
        if cond {
            let f: Final = _dynamic_call::[Final](prog, net, func);
            return final { f.run(); };
        }
        return final { fallback(); };
    }
}

final fn fallback() {}
