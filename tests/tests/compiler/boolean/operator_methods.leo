program test.aleo {
    fn main(a: bool, b: bool) -> bool {
        // unary
        let h: bool = a.not();

        // binary
        let _l: bool = a.and(b);
        let _o: bool = a.eq(b);
        let _v: bool = a.nand(b);
        let _w: bool = a.neq(b);
        let _x: bool = a.nor(b);
        let _y: bool = a.or(b);
        let _ar: bool = a.xor(b);

        return h;
    }

    @noupgrade
    constructor() {}
}
