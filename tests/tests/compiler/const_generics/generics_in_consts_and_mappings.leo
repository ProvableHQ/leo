program b28668.aleo {
    struct Foo::[N: bool] { x: bool }

    const F: Foo::[true] = Foo::[true] { x: true };

    mapping map: Foo::[true] => Foo::[false];

    async transition main(y: u32) -> (bool, Future) {
        return (F.x, f());
    }

    async function f() {
        map.set(Foo::[true] { x: false }, Foo::[false] { x: true });
    }
}
