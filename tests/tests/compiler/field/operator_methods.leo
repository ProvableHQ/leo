program test.aleo {
    fn main(a: field, b: field) -> bool {
        // unary
        let _f: field = a.inv();
        let _g: field = a.neg();
        let _i: field = a.square();
        let _j: field = a.square_root();

        // binary
        let _k: field = a.add(b);
        let _m: field = a.div(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: field = a.mul(b);
        let w: bool = a.neq(b);
        let _z: field = a.pow(b);

        return w;
    }

    @noupgrade
    constructor() {}
}
