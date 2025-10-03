program test.aleo {
    struct Bar { x: u8? }
    struct Foo { b: Bar }

    transition foo(x: u8?, y: [u8?; 3], f: Foo) -> (u8?, u8?, Foo) {
        return (x, x, f);
    }

    async transition bar(x: u8?, f: Foo) {
        return async {};
    }

    function baz(x: u8?) -> u8? {
        return x;
    }
}
