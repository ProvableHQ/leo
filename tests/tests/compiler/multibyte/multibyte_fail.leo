
// The multibyte character ‘ appears in this file, and
// as of commit d579a6fd365bdf0a7ce2a8fb0542347ad77314b9
// this would cause spans in error reporting to be
// bogus.
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

    fn bubble_sort(xs: [u32; 8]) -> [u32; 8] {
       for i: u8 in 0u8..7u8 {
           for j: u8 in 0u8..7u8-i {
               // Move the smaller elements forward
               if arr[j+1u8] < arr[j] {
                   let swap: = arr[j];
                   // let swap = arr[j];
                   arr[j] = arr[j+1u8];
                   arr[j+1u8] = swap;
               }
           }
        return xs;
    }

}
