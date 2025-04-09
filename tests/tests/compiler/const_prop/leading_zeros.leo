// As of commit c16c7bf72ac04622b822db834c4d80da4a48f225,
// group, field, and scalar elements with leading zeros
// were not being parsed correctly.
program test.aleo {
    const F1: field = 01field;
    const F2: field = 001field;
    const F3: field = 0field;
    const F4: field = -0field;
    const F5: field = -001field;
    const F6: field = -1field;

    transition field_tr() -> field {
        return F1 + F2 + F3 + F4 + F5 + F6;
    }

    transition field_tr2() -> field {
        return F5;
    }

    const G1: group = 00288246391group;
    const G2: group = -00288246391group;
    const G3: group = 000group;

    transition group_tr() -> group {
        return G1 + G2 + G3;
    }

    transition group_tr2() -> group {
        return G2;
    }

    const S1: scalar = 0scalar;
    const S2: scalar = 00scalar;
    const S3: scalar = 001scalar;
    const S4: scalar = -0003scalar;
    const S5: scalar = -0002111115437357092606062206234695386632838870926408408195193685246394721360384scalar;

    transition scalar_tr() -> scalar {
        return S1 + S2 + S3 + S4 + S5;
    }

    transition scalar_tr2() -> scalar {
        return S5;
    }

    const I1: i8 = 0i8;
    const I2: i8 = 00i8;
    const I3: i8 = 001i8;
    const I4: i8 = -0003i8;

    transition i8_tr() -> i8 {
        return I1 + I2 + I3 + I4;
    }

    transition i8_tr2() -> i8 {
        return I4;
    }
}
