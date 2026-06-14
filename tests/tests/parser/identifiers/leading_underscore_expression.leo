program test.aleo {
    // Leading-underscore identifiers in expression position are accepted by the parser; if
    // they don't resolve to a declared name, later passes will emit the resolution error.
    fn main() -> u32 {
        return _value;
    }
}

fn foo(_param: u32) -> u32 {
    return _param;
}
