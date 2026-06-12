program test.aleo {
    @noupgrade
    constructor() {}

    fn main(x: bool, y: bool) -> bool {
        let a: bool = x && y;
        let b: bool = y && x;
        let c: bool = x || y;
        let d: bool = y || x;
        let e: bool = x.xor(y);
        let f: bool = y.xor(x);
        let g: bool = x.nand(y);
        let h: bool = y.nand(x);
        let i: bool = x.nor(y);
        let j: bool = y.nor(x);
        return (a && b) && (c && d) && (e && f) && (g && h) && (i && j);
    }
}
