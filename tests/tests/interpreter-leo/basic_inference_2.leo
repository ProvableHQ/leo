/*program child.aleo {
    struct External { x: u32, y: u32 }

    fn main(e: External, public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

import child.aleo;
import credits.aleo;
}*/

const X: u32 = 1;
const Y: u32 = 2;

struct Baz { xxx: u32, yyy: bool }

program test.aleo {
    fn main() {
        let a: [u32; 5] = [1, 2, 3, 4, 5];
        assert(a[1] == 2);

        let b: [u32; 5] = [42; 5];
        assert(b[1] == 42);

        let x = 1u8 + 1;
        let y = 1 + 1u32;
        let z = 2 + 3field + 8;
        let foofoo: u32 = 2;
        let g0: group = 2group * 2;
        let g1: group = 1 * 2group;
        let g2 = 2group * 2;
        let g3 = 1 * 2group;
        let g4: group = 1scalar * 2;
        let g5: group = 2 * 2scalar;
        let g6 = 1scalar * 2;
        let g7 = 2 * 2scalar;
        let xx: u32 = 3 + 3;

        let p0 = 1field.pow(2);
        let p1 = 1.pow(2field);
        let p2: field = 1field.pow(2);
        let p3: field = 1.pow(2field);

        let s: u32 = 1;
        assert(s == 1);

        assert(xx == 6);
        assert(foofoo == 2);
        assert(y == 2);
        assert(z == 13);
        assert_eq(x, 2u8);

        let ff = bar();
        assert(ff == 0);

        assert(foo::[3u32]([1, 2, 3]) == 3);

        assert(Y == 2);
        assert(X == 1);

        let xxx = 0u32;
        let yyy = true;

        let baz = Baz {
            yyy,
            xxx,
        };

        assert(baz.xxx == 0);
        assert(baz.yyy == true);

        let baz2 = Baz { yyy: true, xxx: 5u32 };
        assert(baz2.xxx == 5u32);
        assert(baz2.yyy == true);

        let qq: u32 = 1;
        qq = 2;
        qq = 3;
        assert(qq == 3);

        let uu = (1u32, 2field);
        uu.0 = 4;
        uu.1 = 5;
        assert(uu.0 == 4);
        assert(uu.1 == 5);

        // TODO: Ideally we would be able to test function calls and struct inits from external Leo and Aleo programs
        // This doesn't seem possible today with the current testing infra
        /*let fee = credits.aleo/fee_public(1, 1, 1);
        let c = committee_state {
            commission: 5u8,
            is_open: true,
        };
        assert(c.is_open);
        let e = External { x: 5u32, y: 6u32 };
        assert(e == 5u32);
        assert(y == 6u32);*/
    }
}

fn bar() -> u32 {
    return 0;
}

fn foo::[M: u32](arr: [u32; 3]) -> u32 {
    return arr[M - 1];
}
