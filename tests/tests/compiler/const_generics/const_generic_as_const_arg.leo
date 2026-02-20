program const_generics.aleo {
    fn main() -> field {
        return abc::[0]() + abc::[1]();
    }
}

fn abc::[C: field]() -> field {
    return def::[C]();
}

fn def::[C: field]() -> field {
    return C;
}
