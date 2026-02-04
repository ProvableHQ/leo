
program test.aleo {
    fn baz(foo: u8, bar: u8) -> u8 {
        let a: (u8, u8) = (foo, bar);
        a.0 = a.0 + a.1;
        a.1 = 2u8;
        return a.0 + a.1;
    }

    fn baz2(x: field, y: field) -> (field, field) {
        let tupl: (field, field, u8) = (x, y, 1u8);

        tupl.0 = x + tupl.0;
        tupl.1 = x + tupl.1;

        let tupl2: (field, field) = (tupl.1, tupl.0);

        return tupl2;
    }

    fn baz3(x: field, y: field, z: bool) -> (field, field) {
        let tupl: (field, field) = (x, y);
        if z {
            tupl.0 = tupl.0 + x;
        } else {
            tupl.1 = tupl.1 + y;
        }

        return tupl;
    }

    fn baz4(x: field, y: field, z: bool) -> (field, field) {
        let tupl1: (field, field) = (x, y);
        let tupl2: (field, field) = (y, x);
        let result: (field, field) =  z ? tupl1 : tupl2;
        return result;
    }
}
