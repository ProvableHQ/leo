export interface Iface {
    fn iface_fn::[N: u32](x: u32) -> u32;
}

@no_inline
export fn foo1::[N: u32](x: u32) -> u32 {
    return N;
}

export final fn foo2::[N: u32](x: u32) {}

export fn baz::[N: u32]() {}

program const_generics.aleo {
    fn main() -> Final {
        return final { baz::[1u32](); };
    }

    fn bar1::[N: u32]() {}

    fn bar2::[N: u32]() -> Final {
        return final { baz::[N](); };
    }

    @noupgrade
    constructor() {}
}
