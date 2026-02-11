struct Foo {
    x: u32,
    y: u32
}

program child.aleo {
    fn main(f: Foo) {}
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    fn main() -> child.aleo/Foo {
        return child.aleo/Foo {
            x: 0, y: 0 
        };
    }
}
