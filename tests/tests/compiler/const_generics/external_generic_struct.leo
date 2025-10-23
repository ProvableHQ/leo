program child.aleo {
    struct Bar::[N: u32] {
        arr: [u32; N],
    }

    transition main(b: Bar::[5]) -> Bar::[3] {
        let c = Bar::[4] {
            arr: [b.arr[0]; 4],
        };
        return Bar::[3] {
            arr: [c.arr[1]; 3],
        };
    }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    transition main() {
        // Construct a Bar::[5] from child.aleo
        let bar2 = child.aleo::Bar::[2] {
            arr: [10u32, 20u32],
        };

        // Since child::main always builds Bar::[3] from c.arr[1],
        // and c.arr = [b.arr[0]; 4], we expect all elements == b.arr[0].
        assert(bar2.arr[0] == 10u32);
        assert(bar2.arr[1] == 20u32);
    }
}
