export struct Foo::[N: bool] {
    x: bool,
}

export const F: Foo::[true] = Foo::[true] { x: true };

program b28668.aleo {
    mapping map: Foo::[true] => Foo::[false];

    fn main(y: u32) -> (bool, Final) {
        return (
            F.x,
            final {
                f();
            },
        );
    }

    @noupgrade
    constructor() {}
}

export final fn f() {
    map.set(Foo::[true] { x: false }, Foo::[false] { x: true });
}
