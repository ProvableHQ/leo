program test.aleo {
    fn main(a: group, b: group) -> bool {
        // unary
        let _e: group = a.double();
        let _g: group = a.neg();

        // binary
        let _j: group = a.add(b);
        let _o: bool = a.eq(b);
        let _t: group = a.mul(2scalar);
        let _q: group = 2scalar.mul(a);
        let _w: bool = a.neq(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
