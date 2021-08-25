/*
namespace: ParseStatement
expectation: Fail
*/

let mut x = expr;

let mut x = ();

let mut x = x+y;

let mut x = (x,y);

let mut x = x();


const mut x = expr;

const mut x = ();

const mut x = x+y;

const mut x = (x,y);

const mut x = x();


let mut x: u32 = expr;

let mut x: u32 = ();

let mut x: u32 = x+y;

let mut x: u32 = (x,y);

let mut x: u32 = x();


const mut x: u32 = expr;

const mut x: u32 = ();

const mut x: u32 = x+y;

const mut x: u32 = (x,y);

const mut x: u32 = x();


let (x,y,,) = ();

let (,x,y) = ();

let (x,,y) = ();


let x: [u32: aa] = expr;

let x: [u32; _ 2] = expr;
