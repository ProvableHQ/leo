function fibonacci(i: u32) -> u32 {
    if i == 0 {
        return 0
    } else if i == 1 {
        return 1
    } else {
        return fibonacci(i - 1) + fibonacci(i - 2)
    }
}

// The 'fibonacci' main function.
function main() -> u32 {
    return fibonacci(1)
}


Function mutateNoLet(b: bool) {
  let mut a = 5;
  if b {
    // must be turned into conditional expression
    a = 0;
    // a = if b ? 0 : a;
  } else {
    a = 3;
    // a = if b ? a : 3;
  }
}