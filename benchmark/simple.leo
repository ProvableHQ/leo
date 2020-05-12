// Basic Pedersen hash function
// 1 window x 3 bits

struct PedersenCRHParameters {
  bases: fe[3]
}

function main(bits: private bool[3]) -> fe {
  let params = PedersenCRHParameters { bases: [1fe, 1fe, 1fe] };

  let res = 0fe;

  for bit in 0..3 {
    //if (bits[bit]) {
      res += params.bases[bit];
    //}
  }

  return res
}