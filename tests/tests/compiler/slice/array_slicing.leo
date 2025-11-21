
program test.aleo {
    transition full() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[..];
        assert(arr[1] == arr2[0]);
    }

    transition end_exclusive() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[..2];
        let arr3 = arr[..3];
        assert(arr[1] == arr2[0]);
    }

    transition end_inclusive() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[..=2];
        assert(1 == arr2[0]);
    }

    transition start() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[2..];
        assert(3 == arr2[0]);
    }

    transition start_end_exclusive() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[1..3];
        assert(3 == arr2[1]);
    }

    transition start_end_inclusive() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[1..=3];
        assert(4 == arr2[2]);
    }

    transition expression_indexing() {
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let arr2 = arr[(1u32 + 0u32)..=(3u32 - 1u32)];
        assert(2 == arr2[1]);
    }
}
