
struct S {
    x: u8,
    y: u32,
}


struct T {
    x: u8,
    y: [S; 3],
    z: S,
}

final fn f_finalize(x: u8) {
    assert_eq(x, x);
}

program test.aleo {
    fn simple_struct(a: S) -> S {
        a.x = 1u8;
        a.y = a.y + 2u32;
        return a;
    }

    fn nested(a: [[[bool; 2]; 2]; 2]) -> [[[bool; 2]; 2]; 2] {
        a[0u8][0u8][0u8] = true;
        a[1u8][1u8] = [true, false];
        return a;
    }

    fn nested2(s: S, t: T) -> (S, T) {
        s.y = s.y + 1u32;
        t.z = s;
        t.y[1u8] = s;
        t.y[0u8].x = 1u8;
        return (s, t);
    }

    fn with_tuples(s: S, t: T) -> (S, T) {
        let st: (S, T) = (s, T { x: t.x, y: t.y, z: s});
        st.1 = t;
        st.0.x = 1u8;
        st.1.y[1u8] = s;
        return st;
    }

    fn in_conditional(t: T, x: bool) -> T {
        if x {
            t.x = 1u8;
        } else {
            t.z = S {
                x: 2u8,
                y: 3u32,
            };
        }
        return t;
    } 

    record R {
        owner: address,
        x: u8,
    }

    fn f_record() -> (R, Final) {
        let fut: Final = final { f_finalize(1u8); };
        let r: R = R {
            owner: self.signer,
            x: 1u8,
        };
        r.x = 2u8;
        return (r, fut);
    }

    fn in_loops(xs: [u8; 8]) -> [u8; 8] {
        for i: u8 in 0u8..7u8 {
            for j: u8 in 0u8..7u8 - i {
                if xs[j] < xs[j+1u8] {
                    let tmp: u8 = xs[j];
                    xs[j] = xs[j+1u8];
                    xs[j+1u8] = tmp;
                }
            }
        }
        return xs;
    }
}
