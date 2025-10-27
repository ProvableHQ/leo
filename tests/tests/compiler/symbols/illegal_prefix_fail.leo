program child.aleo {
    record Foo {
        owner: address,
        val: u32,
    }

    transition bar() {}
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // This shouldn't conflict with child.aleo/Foo
    record Foo {
        owner: address,
    }

    // Foo and FooBar are both prefixes. Shouldn't conflict with child.aleo/Foo though.
    record FooBarBaz {
        owner: address,
    }

    // Foo is a prefix
    record FooBar {
        owner: address,
    }

    transition foo () {}
}
