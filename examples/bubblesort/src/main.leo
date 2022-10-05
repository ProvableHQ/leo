program bubblesort.aleo {
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

    transition bubble_sort(
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

        // (i, j) = (0, 1).
        // Move the smaller elements forward.
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }

        // (i, j) = (0, 2).
        // Move the smaller elements forward.
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }

        // (i, j) = (0, 3).
        // Move the smaller elements forward.
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }

        // (i, j) = (0, 4).
        // Move the smaller elements forward.
        if arr5 < arr4 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr4;
            arr4 = arr5;
            arr5 = temp;
        }

        // (i, j) = (0, 5).
        // Move the smaller elements forward.
        if arr6 < arr5 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr5;
            arr5 = arr6;
            arr6 = temp;
        }

        // (i, j) = (0, 6).
        // Move the smaller elements forward.
        if arr7 < arr6 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr6;
            arr6 = arr7;
            arr7 = temp;
        }

        // (i, j) = (1, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }

        // (i, j) = (1, 1).
        // Move the smaller elements forward.
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }

        // (i, j) = (1, 2).
        // Move the smaller elements forward.
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }

        // (i, j) = (1, 3).
        // Move the smaller elements forward.
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }

        // (i, j) = (1, 4).
        // Move the smaller elements forward.
        if arr5 < arr4 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr4;
            arr4 = arr5;
            arr5 = temp;
        }

        // (i, j) = (1, 5).
        // Move the smaller elements forward.
        if arr6 < arr5 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr5;
            arr5 = arr6;
            arr6 = temp;
        }

        // (i, j) = (2, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }

        // (i, j) = (2, 1).
        // Move the smaller elements forward.
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }

        // (i, j) = (2, 2).
        // Move the smaller elements forward.
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }

        // (i, j) = (2, 3).
        // Move the smaller elements forward.
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }

        // (i, j) = (2, 4).
        // Move the smaller elements forward.
        if arr5 < arr4 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr4;
            arr4 = arr5;
            arr5 = temp;
        }

        // (i, j) = (3, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }

        // (i, j) = (3, 1).
        // Move the smaller elements forward.
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }

        // (i, j) = (3, 2).
        // Move the smaller elements forward.
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }

        // (i, j) = (3, 3).
        // Move the smaller elements forward.
        if arr4 < arr3 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr3;
            arr3 = arr4;
            arr4 = temp;
        }

        // (i, j) = (4, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }

        // (i, j) = (4, 1).
        // Move the smaller elements forward.
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }

        // (i, j) = (4, 2).
        // Move the smaller elements forward.
        if arr3 < arr2 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr2;
            arr2 = arr3;
            arr3 = temp;
        }

        // (i, j) = (5, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }

        // (i, j) = (5, 1).
        // Move the smaller elements forward.
        if arr2 < arr1 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr1;
            arr1 = arr2;
            arr2 = temp;
        }

        // (i, j) = (6, 0).
        // Move the smaller elements forward.
        if arr1 < arr0 {
            // Swap the elements at indexes ‘j‘ and ‘j+1‘
            let temp: u32 = arr0;
            arr0 = arr1;
            arr1 = temp;
        }

        return (arr0, arr1, arr2, arr3, arr4, arr5, arr6, arr7);
    }
}
