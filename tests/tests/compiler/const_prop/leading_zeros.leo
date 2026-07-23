// As of commit c16c7bf72ac04622b822db834c4d80da4a48f225,
// group, field, and scalar elements with leading zeros
// were not being parsed correctly.
export const F1: field = 008444461749428370424248824938781546531375899335154063827935233455917409239041field;
export const F2: field = 008444461749428370424248824938781546531375899335154063827935233455917409239043field;
export const F3: field = -008444461749428370424248824938781546531375899335154063827935233455917409239043field;
export const F4: field = 008444461749428370424248824938781546531375899335154063827935233455917409239039field;
export const F5: field = -008444461749428370424248824938781546531375899335154063827935233455917409239039field;
export const F6: field = 01field;
export const F7: field = 000field;

export const G1: group = 008444461749428370424248824938781546531375899335154063827935233455917409239041group;
export const G2: group = 008444461749428370424248824938781546531375899335154063827935233455917409239043group;
export const G3: group = -008444461749428370424248824938781546531375899335154063827935233455917409239043group;
export const G4: group = 008444461749428370424248824938781546531375899335154063827935233455917409239039group;
export const G5: group = -008444461749428370424248824938781546531375899335154063827935233455917409239039group;
export const G6: group = 000group;
export const G7: group = 00288246391group;
export const G8: group = -00288246391group;

export const S1: scalar = 0scalar;
export const S2: scalar = 00scalar;
export const S3: scalar = 001scalar;
export const S4: scalar = -0003scalar;
export const S5: scalar = -0002111115437357092606062206234695386632838870926408408195193685246394721360384scalar;

export const I1: i8 = 0i8;
export const I2: i8 = 00i8;
export const I3: i8 = 001i8;
export const I4: i8 = -0003i8;

program test.aleo {
    fn field_tr() -> field {
        return F1 + F2 + F3 + F4 + F5 + F6 + F7;
    }

    fn field_tr2() -> field {
        return F3;
    }

    fn group_tr() -> group {
        return G1 + G2 + G3 + G4 + G5 + G6 + G7 + G8;
    }

    fn group_tr2() -> group {
        return G3;
    }

    fn scalar_tr() -> scalar {
        return S1 + S2 + S3 + S4 + S5;
    }

    fn scalar_tr2() -> scalar {
        return S5;
    }

    fn i8_tr() -> i8 {
        return I1 + I2 + I3 + I4;
    }

    fn i8_tr2() -> i8 {
        return I4;
    }

    @noupgrade
    constructor() {}
}
