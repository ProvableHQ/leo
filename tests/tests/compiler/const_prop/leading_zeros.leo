
// As of commit c16c7bf72ac04622b822db834c4d80da4a48f225,
// group, field, and scalar elements with leading zeros
// were not being parsed correctly.
program test.aleo {
    const A: field = 01field;

    transition a() -> field {
        return A + 1field;
    }

    const B: group = 00288246391group;

    transition b() -> group {
        return B + 0group;
    }

    const C: scalar = 0001scalar;

    transition c() -> scalar {
        return C + 1scalar;
    }

    const D: i8 = 01i8;

    transition d() -> i8 {
        return D + 1i8;
    }

    const E: field = 0000field;

    transition e() -> field {
        return E + 1field;
    }
}
