program test.aleo {
   transition flatten(a: [[bool; 2]; 2]) -> [bool; 4] {
      let b = a[0] + a[1];
      let c = [a[0][0], a[0][1], a[1][0], a[1][1]];
      let d = flatten2::[2](a);
      let e = generic_flatten::[2, 2](a);

      assert_eq(b, c);
      assert_eq(b, d);
      assert_eq(b, e);

      return b;

   }

    inline flatten2::[M: u32](bits: [[bool; M]; 2]) -> [bool; 2 * M] {
        return bits[0] + bits[1];
    }

    inline generic_flatten::[M: u32, N: u32](bits: [[bool; M]; N]) -> [bool; M * N] {
        let flattened = [false; M * N];
        for i in 0u32..N {
          for j in 0u32..M {
            flattened[i * M + j] = bits[i][j];
          }
        }
        return flattened;
    }
}