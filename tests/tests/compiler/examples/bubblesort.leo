program test.aleo {
    // Executes the bubble sorting algorithm.
    // The original algorithm is given below.
    //
    // for i in 0..7 {
    //     for j in 0..7-i {
    //         // Move the smaller elements forward
    //         if arr[j+1] < arr[j] {
    //             // Swap the elements at indexes ‘j‘ and ‘j+1‘
    //             let swap = arr[j];
    //             arr[j] = arr[j+1];
    //             arr[j+1] = swap;
    //         }
    //     }
    // }
    //
    // Note that the implementation below uses tuples instead of arrays.
    // The implementation also manually unrolls the loop.
    fn bubble_sort(
        arr0: u32,
        arr1: u32,
        arr2: u32,
        arr3: u32,
        arr4: u32,
        arr5: u32,
        arr6: u32,
        arr7: u32,
    ) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        // Unroll the loops.
        // (i, j) = (0, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }
        if arr5 < arr4 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr4;
            arr4 = arr5;
            arr5 = temp;
        }
        if arr6 < arr5 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr5;
            arr5 = arr6;
            arr6 = temp;
        }
        if arr7 < arr6 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr6;
            arr6 = arr7;
            arr7 = temp;
        }
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }
        if arr5 < arr4 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr4;
            arr4 = arr5;
            arr5 = temp;
        }
        if arr6 < arr5 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr5;
            arr5 = arr6;
            arr6 = temp;
        }
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }
        if arr5 < arr4 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr4;
            arr4 = arr5;
            arr5 = temp;
        }
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }
        return (arr0, arr1, arr2, arr3, arr4, arr5, arr6, arr7);
    }
}
