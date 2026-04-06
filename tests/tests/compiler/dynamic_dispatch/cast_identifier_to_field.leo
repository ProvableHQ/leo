// Cast an identifier literal to a field.
program test.aleo {
    fn main() -> field {
        let f: field = 'foo' as field;
        return f;
    }
}
