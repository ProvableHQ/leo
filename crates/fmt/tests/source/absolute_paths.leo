const X: u32 = 5u32;

struct Shared {
    val: u32,
}

struct Gen::[N: u32] {
    vals: [u32; N],
}

fn shared(v: u32) -> Shared {
    return Shared { val: v };
}

program test.aleo {
    fn main() -> u32 {
        let v = ::shared( 1u32 ).val;
        let s = ::Shared { val:  ::X };
        let g = ::Gen::[2] { vals: [1u32,  2u32] };
        return v + s.val + g.vals[0u32];
    }

    @noupgrade
    constructor() {}
}
