# arc20.aleo
A draft design of arc-20

## Summary
This program is designed to provide a token interface similar to the ERC-20 standard.

**Data Structures**

- **Record Type**
  - **Credential**: `Credential` is a `record` type that includes information about the `token_id`, `owner`, `total_supply`and `supplied` indicates the amount already issued.
  - **Token**: `Token` is a `record` type that includes information about the `owner`, `token_id` , `amount`.
- **Struct**
  - **Order**: `Order` is a struct to store the exchange information.
  - **Pair**: `Pair` is a struct contains `token_id` and `address`.  As leo doesn't support two-dimensional arrary yet, we can work around it. This is the `key` type of `mapping balance`.
- **Mapping**
  - **balance**: `mapping(token_id, address) => u64` stores the public balance on chain.
  - **exist**: `mapping exist: u64 => bool` indicates whether certain `token_id` is registered or not. Maybe we can store the `(address, total_supply)` as the value.

**Core Transitions**

- **register**: This transiton will create a `Credential` which contains a unique `token_id` and determine the `total_supply` at the same time.
- **drop**: This transition destroy a `Credential`.
- **mint**: This transition need a `Credential` as one of inputs to mint the token which has the same `token_id` with `Credential`.
- **transfer**: This transition need a `Token` as one of inputs to transfer tokens to others.
- **split**: This transition splits `token` into two parts.
- **burn**: This transition will destroy the token.



### Running the program

##### 1. Register a specific token

```shell
leo run register 1u64 10000000000u64
```

**Output**

```
{
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  total_supply: 10000000000u64.private,
  supplyed: 0u64.private,
  _nonce: 896222600206158262508858421184517354181119302763164022413143984243775956809group.public
}
```

##### 2.Use the `Credential Record` to mint 100 token to a specific address

```shell
leo run mint '{
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  total_supply: 10000000000u64.private,
  supplyed: 0u64.private,
  _nonce: 896222600206158262508858421184517354181119302763164022413143984243775956809group.public
}' aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t 100u64
```

**Output**

```
 • {
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  total_supply: 10000000000u64.private,
  supplyed: 100u64.private,
  _nonce: 3917718114751162483393640984554525533557039900850940745568933438061470407465group.public
}
 • {
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  amount: 100u64.private,
  _nonce: 3274251857893957222912449746604915309714653806460058212946776991645960573812group.public
}
```

##### 3. Use the  `Record Token` transfer token to a specific address

```shell
leo run transfer '{
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  amount: 100u64.private,
  _nonce: 7496445556149988450618064416294485477321137882513878392977802376165102315258group.public
}' aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t 50u64
```

**Output**

```
 • {
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  amount: 50u64.private,
  _nonce: 1102369324326279891287336228512080501766782857066736461415823376899330136629group.public
}
 • {
  owner: aleo1k8x09g9shjnvsdet9dhwxm30jyeppnehqu8hhlznfk0pejmmvgxq99h54t.private,
  token_id: 1u64.private,
  amount: 50u64.private,
  _nonce: 2968419129482623878255943787752853399163961238577773730962559144711742326729group.public
}
```

