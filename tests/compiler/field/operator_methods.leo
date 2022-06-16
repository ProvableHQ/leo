/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/fields.in
*/

function main(a: field, b: field) -> bool {
   // unary
   let f: i8 = a.inv();
   let g: i8 = a.neg();
   let i: i8 = a.square();

   // binary
   let j: i8 = a.add(b);
   let m: i8 = a.div(b);
   let o: bool = a.eq(b);
   let p: bool = a.ge(b);
   let q: bool = a.gt(b);
   let r: bool = a.le(b);
   let s: bool = a.lt(b);
   let t: i8 = a.mul(b);
   let w: bool = a.neq(b);
   let z: i8 = a.pow(2u8);
   let aa: i8 = a.pow(2u16);
   let ab: i8 = a.pow(2u32);

   return f;
}