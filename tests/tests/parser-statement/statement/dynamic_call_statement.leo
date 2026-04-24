
let result: u32 = Adder@(target)::sum(1u32, 2u32);

return Adder@('foo')::sum(x, y);

let total: u64? = Counter@(target)::total;

let n: u32 = Logs@(target)::entries.len();

let first: u64? = Logs@(target, 'aleo')::entries.get(0u32);
