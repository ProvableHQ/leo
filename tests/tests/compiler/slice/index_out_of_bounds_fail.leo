
program test.aleo {
    transition index_out_of_bounds() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[1..3];
        assert(arr[1] == arr2[2]);   
    }
}
