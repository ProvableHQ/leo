
function foo(n: u8) -> u8 {
    return bar(n);
}

function bar(n: u8) -> u8 {
    return foo(n);
}

program test.aleo {
    transition baz(n: u8) -> u8 {
        return bax(n);
    }

    transition bax(n: u8) -> u8 {
        return baz(n);
    }
}
