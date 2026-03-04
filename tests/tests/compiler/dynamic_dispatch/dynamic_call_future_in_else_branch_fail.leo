program test.aleo {
    fn main(cond: bool, prog: field, net: field, func: field) -> Final {
        if cond {
            return final { helper(); };
        } else {
            let f: Final = _dynamic_call::[Final](prog, net, func);
            return final { f.run(); };
        }
    }
}

final fn helper() {}
