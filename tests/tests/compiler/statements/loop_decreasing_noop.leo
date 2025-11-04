
program test.aleo {
    transition main(x: u32) -> bool {

        let count: i8 = 0i8;
        for i: i8 in 10i8..5i8 {
            count += 1i8;
            for j: i8 in 10i8..6i8 {
                count += j;
            }
        }

        for i in 10u32..10u32 {
            count += 1i8;
        }

        for i in 0u32..2u32 {
            for j in 0u32..0u32 {
                count += 1i8;
            }
        }

        return true;
    }
}
