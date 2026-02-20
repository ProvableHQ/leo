
program test.aleo {
    fn main(k: bool) -> bool {
        let b: bool = true;
        b &&= k;

        return b;
    }}
