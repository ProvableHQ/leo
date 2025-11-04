program test.aleo {
    transition slice_out_of_bounds(a: [bool; 32]) -> bool {
        let b = a[(2u32 - 1 - 1)..(8u32 * 8 * 3)];
        return true;
    }
}