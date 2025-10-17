
program test.aleo {
    transition main() -> bool {
      return self.address;
    }

    transition next() -> bool {
      let a = self.address;
      return a;
    }
}
