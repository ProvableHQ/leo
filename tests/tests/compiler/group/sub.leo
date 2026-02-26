
program test.aleo {
    fn main(a: group, b: group, c: group) -> bool {
        assert(a - b == c);

        return a - b == c;
    }}
