// A struct may now define methods named after built-in collection/optional/signature methods
// (`get`, `set`, `unwrap`, `verify`, ...). These are resolved by the receiver's type, so on a
// struct/record they dispatch to the user method rather than a built-in intrinsic.
export struct Bag {
    items: u32,
}

impl Bag {
    fn get(self) -> u32 {
        return self.items;
    }

    fn set(self, v: u32) -> Bag {
        return Bag { items: v };
    }

    fn unwrap(self) -> u32 {
        return self.items;
    }

    fn verify(self, a: u32, b: u32) -> bool {
        return self.items == a + b;
    }
}

program test.aleo {
    fn main(a: u32) -> u32 {
        let b: Bag = Bag { items: a };
        let g: u32 = b.get();
        let c: Bag = b.set(g + 1u32);
        let u: u32 = c.unwrap();
        let ok: bool = c.verify(1u32, 2u32);
        return u + (ok ? 1u32 : 0u32);
    }

    @noupgrade
    constructor() {}
}
