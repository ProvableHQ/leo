program test.aleo {
   transition flatten(a: [[bool; 2]; 2]) -> [bool; 4] {
      let b = a[0] + a[1];
      let c = [a[0][0], a[0][1], a[1][0], a[1][1]];

      assert_eq(b, c);
      return b;

   }
}
