program const_generics.aleo {
    fn main() -> field {
        return abc::[0]() + abc::[1]();
    }
}

@inline
fn abc::[C: field]() -> field {
    return def::[C]();
}

@inline
fn def::[C: field]() -> field {
    return C;
}
