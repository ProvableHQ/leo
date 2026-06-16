program test.aleo {
    fn main(a: bool, b: bool) -> (bool, bool) {
        let x: bool = a.nand(true);
        let y: bool = b.nor(false);
        return (x, y);
    }

    @noupgrade
    constructor() {}
}
