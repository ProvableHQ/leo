program const_generics.aleo {
    fn main() -> field {
        return 0;
    }

    fn main2() -> Final {
        return final { foo(); };
    }
}

final fn foo() {}

fn bar(x: field) -> field { return x; }

// Ensure we don't error out on functions that are not accessible from program entry points
fn abc::[C: field]() -> field {
    return def::[C]();
}

fn def::[C: field]() -> field {
    return C;
}
