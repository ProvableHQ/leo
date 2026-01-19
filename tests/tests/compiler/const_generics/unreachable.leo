program const_generics.aleo {
    fn main() -> field {
        return 0;
    }

    async fn main2() -> Future {
        return foo();
    }
}

async fn foo() {}

fn bar(x: field) -> field { return x; }

// Ensure we don't error out on functions that are not accessible from program entry points
@inline
fn abc::[C: field]() -> field {
    return def::[C]();
}

@inline
fn def::[C: field]() -> field {
    return C;
}
