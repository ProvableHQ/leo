program child.aleo {
    record Foo {
        owner: address,
        val: u32,
    }

    transition bar() {}
}

// --- Next Program --- //

import child.aleo;

program fooaleobar.aleo {
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

    // Foo is a prefix and also it has aleo in the name. Also, the fields have aleo in their names.
    record FooaleoBar {
        owner: address,
        aaleob: u32,
        aleob: u32,
        daleo: u32,
    }

    transition foo () {}
}
