/*
namespace: ParseStatement
expectation: Pass
*/

let x = expr;

let x = ();

let x = x+y;

let x = (x,y);

let x = x();


const x = expr;

const x = ();

const x = x+y;

const x = (x,y);

const x = x();


let x: u32 = expr;

let x: u32 = ();

let x: u32 = x+y;

let x: u32 = (x,y);

let x: u32 = x();


const x: u32 = expr;

const x: u32 = ();

const x: u32 = x+y;

const x: u32 = (x,y);

const x: u32 = x();






let (x, y) = expr;

let (x, y) = ();

let (x, y) = x+y;

let (x, y) = (x,y);

let (x, y) = x();


const (x, y) = expr;

const (x, y) = ();

const (x, y) = x+y;

const (x, y) = (x,y);

const (x, y) = x();


let (x, y): u32 = expr;

let (x, y): u32 = ();

let (x, y): u32 = x+y;

let (x, y): u32 = (x,y);

let (x, y): u32 = x();


const (x, y): u32 = expr;

const (x, y): u32 = ();

const (x, y): u32 = x+y;

const (x, y): u32 = (x,y);

const (x, y): u32 = x();


let (x,y,) = ();

let (x,) = ();

let x: [[u8; 2]; 2] = [[0,0], [0,0]];

let x: [u8; (2, 2)] = [[0,0], [0,0]];


let x: address = aleo15u4r0gzjtqzepkgurgn7p3u5kkhs9p74rx6aun3uh2s5std6759svgmg53;
