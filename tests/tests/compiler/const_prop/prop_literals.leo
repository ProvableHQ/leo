
program test.aleo {
    const A: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;

    const B: field = 0field;

    const C: [address; 2] = [A, A];

    transition f1() -> address {
        let a: address = A;
        return a;
    }

    transition f2() -> field {
        let b: field = B;
        return b;
    }

    transition f3() -> [address; 2] {
        let c: [address; 2] = C;
        return c;
    }
}
