program test.aleo {
    fn select(s: bool, a: identifier, b: identifier) -> identifier {
        return s ? a : b;
    }

    fn select_literal(s: bool) -> identifier {
        return s ? 'foo' : 'bar';
    }
}
