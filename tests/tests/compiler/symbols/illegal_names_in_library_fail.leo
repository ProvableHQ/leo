program childaleo.aleo {
    record Foo {
        owner: address,
    }

    record FooBarBaz {
        owner: address,
    }

    record FooBar {
        owner: address,
    }

    record FooaleoBar {
        owner: address,
        aaleob: u32,
        aleob: u32,
        daleo: u32,
    }
    transition bar() {}
}

// --- Next Program --- //

import childaleo.aleo;

program foo.aleo {
    transition foo () {}
}
