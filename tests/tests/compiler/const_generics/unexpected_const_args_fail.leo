const M: u32 = 5;

struct Goo::[P: u32] {
    g: M::[6],
    f: baz::[5],
}

program test.aleo {
    mapping baz: u32 => u32;
    fn main(g: Goo::[7u32]) -> u32 {
        let x = 5u32;
        let y: x::[5] = [3];

        return 0u32;
    }
}
