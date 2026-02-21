
program test.aleo {
    fn main(a: group, b: group, c: group) -> bool {
        let r: group = true ? a : b;

        assert(r == c);
        return r == c;
    }}
