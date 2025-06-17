program test.aleo {
    transition binary_ops() {
        let a0 = 1;
        let a1 = 1 && true;
        let a2 = true || 1;
        let a3 = 1.nand(1);
        let a4 = 1.nor(1).nor(1);

        let b0 = 1 & true;
        let b1 = true | 1;
        let b2 = 1 ^ 1;

        let c0 = 1 + true;
        let c1 = 1 - 1;
        let c3 = 1 * 1;
        let c4 = 1 % 1;
        let c5 = 1.rem_wrapped(1);
        let c6 = 1.mod(1);
        let c7 = 1.pow(1);

        let d0 = 1 == 1;
        let d1 = 1 != 1;
        let d2 = 1 > 1;
        let d3 = 1 < 1;
        let d4 = 1 >= 1;
        let d5 = 1 <= 1;

        let e0 = 1.add_wrapped(1);
        let e1 = 1.sub_wrapped(1);
        let e2 = 1.div_wrapped(1);
        let e3 = 1.mul_wrapped(1);

        let f0 = 1.shl(1);
        let f1 = 1.shl_wrapped(1);
        let f2 = 1.shr(1);
        let f3 = 1.shr_wrapped(1);
        let f4 = 1.pow_wrapped(1);
    }

    transition unary_ops() {
        let b0 = -1.abs();
        let b1 = -1.abs_wrapped();
        let b2 = 0.double();
        let b3 = 2.inv();
        let b4  = -2;
        let b5  = !2;
    }
}
