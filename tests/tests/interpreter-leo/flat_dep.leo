struct Merge {
    a: util::Data::[3],
    b: config::Data,
}

program test.aleo {
    fn main() -> u32 {
        let val1 = util::X;
        let val2 = config::X;

        let d1 = util::Data::[3] {
            values: [10, 20, 30],
        };

        let d2 = config::Data {
            values: [100; config::X],
        };

        let res1 = util::helper();
        let res2 = config::helper();
        let boosted = shared::boost(val1 + val2);

        return val1 + val2
             + d1.values[1] + d2.values[2]
             + res1 + res2 + boosted;
    }
}

// --- Next Module: util.leo --- //

const X: u32 = 3;

struct Data::[N: u32] {
    values: [u32; N],
}

fn helper() -> u32 {
    let d = Data::[X] {
        values: [1, 2, 3],
    };
    return d.values[0];
}

// --- Next Module: config.leo --- //

const X: u32 = 4;

struct Data {
    values: [u32; X],
}

fn helper() -> u32 {
    let d = Data {
        values: [5; X],
    };
    return d.values[1];
}

// --- Next Module: shared.leo --- //

fn boost(n: u32) -> u32 {
    // Only uses values accessible within this module or passed in
    return n * 2;
}
