function main(const x: [i16; (2, 2, 3)]){
  console.log("x: {}", x);

  let y: [i16; (2, 2, 3)] = [0i16; (2, 2, 3)];
  console.log("y: {}", y);

  console.assert(x[0][0][0] == y[0][0][0]);
  console.assert(x[1][1][2] == y[1][1][2]);
}