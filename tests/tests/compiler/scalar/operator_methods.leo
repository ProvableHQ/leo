program test.aleo {
    fn main(a: scalar, b: scalar) -> bool {
        // binary
        let _j: scalar = a.add(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        // let t: group = 2group.mul(b);
        // let u: group = a.mul(2group);
        let _w: bool = a.neq(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
