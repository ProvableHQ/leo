struct Foo::[N: address] { x: u32 }

struct Goo::[P: u32] {
    z: Foo::[self.caller],
}

program b28668.aleo {
    fn main(y: u32) -> u32 {
        let g: Goo::[y] = Goo::[y] {
            z: Foo::[self.caller] {
                x: 5,
            }
        };
        return g.z.x;
    }
}
