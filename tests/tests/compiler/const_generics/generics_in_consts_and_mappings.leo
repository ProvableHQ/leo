struct Foo::[N: bool] { x: bool }

const F: Foo::[true] = Foo::[true] { x: true };

program b28668.aleo {
    mapping map: Foo::[true] => Foo::[false];

    fn main(y: u32) -> (bool, Final) {
        return (F.x, final { f(); });
    }
}

final fn f() {
    map.set(Foo::[true] { x: false }, Foo::[false] { x: true });
}
