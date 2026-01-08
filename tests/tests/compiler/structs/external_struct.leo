
program child.aleo {
    struct Foo {
        x: u32,
        y: u32
    }

    transition main(f: Foo) {}
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    transition main() -> child.aleo/Foo {
        return child.aleo/Foo {
            x: 0, y: 0 
        };
    }
}
