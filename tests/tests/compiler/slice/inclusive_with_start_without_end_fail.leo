
program test.aleo {
    transition inclusive_with_start_no_end() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[1..=];
        assert(arr[1] == arr2[0]);   
    }
}
