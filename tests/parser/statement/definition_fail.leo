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

let x: [u8; (2,,)] = [[0,0], [0,0]];


let x: const = expr;

const x: let = expr;

let x: mut = expr;

let

let x

let x:

let x = (a, y]);

let = 1u8;

let;

let x 1u8;

let x: u8;

let x: u8

let x: = 1;

let x: [u8] = 1;

let x: [u8;

let x: [u8; 1u8] = [1,

let dbg: u8 = ];