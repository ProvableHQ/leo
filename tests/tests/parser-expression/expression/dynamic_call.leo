
Adder@(target)::sum(x, y)

Adder@('foo')::sum(x, y)

Adder@('foo', 'aleo')::sum(x, y)

Adder@(target)::sum()

Bank@(target)::balances.get(key)

Bank@('foo')::balances.contains(key)

Bank@(target, 'aleo')::balances.get_or_use(key, 0u64)

Counter@(target)::total

Counter@(target, 'aleo')::total

Logs@(target)::entries.get(i)

Logs@(target)::entries.len()

Logs@(target, 'aleo')::entries.len()
