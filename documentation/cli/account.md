---
id: cli_account
title: ""
sidebar_label: Account
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_account, account, sign, verify, signature, private_key, view_key, address"

# `leo account`

The `leo account` command is used to create and manage Aleo accounts, as well as sign and verify messages

:::warning
We urge you to exercise caution when managing your private keys. `leo account` cannot be used to recover lost keys.
:::

# Subcommands

- [`new`](#leo-account-new) - Generates a new Aleo account.
- [`import`](#leo-account-import) - Derive an Aleo account from a private key.
- [`sign`](#leo-account-sign) - Sign a message using your Aleo private key.
- [`verify`](#leo-account-verify) - Verify a message and signature from an Aleo address.
- [`decrypt`](#leo-account-decrypt) - Decrypt record ciphertexts using your Aleo private key or view key.

&nbsp;

---

## `leo account new`

[Back to Top](#subcommands)

Use this command to generate a private key, view key, and address for a new Aleo account.

The output should look like this:

```bash title="console output:"
  Private Key  APrivateKey1zkp...
     View Key  AViewKey1...
      Address  aleo1...
```

### Flags

#### `--seed <SEED>`

#### `-s <SEED>`

Specifies a particular numeric value to use as the seed for the random number generator (RNG)

#### `--write`

#### `-w`

Writes the generated private key to a `.env` file in the current working directory (`./`)

#### `--discreet`

Print sensitive information (such as private key) discreetly to an alternate screen

&nbsp;

---

## `leo account import`

[Back to Top](#subcommands)

Use this command to derive the view key and address for an Aleo account from a private key.

To import an existing Aleo account, run the following command:

```bash
leo account import <PRIVATE_KEY>
```

where `<PRIVATE_KEY>` is your private key.

### Flags

#### `--write`

#### `-w`

Writes the generated private key to a `.env` file in the current working directory (`./`)

#### `--discreet`

Print sensitive information (such as private key) discreetly to an alternate screen

&nbsp;

---

## `leo account sign`

[Back to Top](#subcommands)

Use this command to sign a message using your Aleo private key.

Assuming either the current working directory is a Leo project or the `$PRIVATE_KEY` environment variable has been set, you can sign a message using the following command:

```bash
leo account sign --message <MESSAGE>
```

### Flags

#### `--message <MESSAGE>`

#### `-m <MESSAGE>`

:::info
This flag is required!
:::

Specifies the message to be signed.

---

#### `--private-key <PRIVATE_KEY>`

Explicitly specifies the private key to sign the message with. Overrides any private key in `.env` file or `$PRIVATE_KEY` environment variable.

#### `--private-key-file <PRIVATE_KEY_FILE>`

Alternative way to explicitly specify the private key by reading from a text file at path `<PRIVATE_KEY_FILE>`. Overrides any private key in `.env` file or `$PRIVATE_KEY` environment variable.

#### `--raw`

#### `-r`

Parses the message as bytes instead of Aleo literals.

&nbsp;

---

## `leo account verify`

[Back to Top](#subcommands)

Use this command to verify a message and signature from an Aleo address.

To verify a message, run the following command

```bash
leo account verify --address <ADDRESS> --signature <SIGNATURE> --message <MESSAGE>
```

where `<MESSAGE>` is the message, `<SIGNATURE>` is the signature of that message, and `<ADDRESS>` is the address of the account that generated the signature.

### Flags

#### `--address <ADDRESS>`

#### `-a <ADDRESS>`

:::info
This flag is required!
:::

Specifies the address of the account that generated the signature.

#### `--signature <SIGNATURE>`

#### `-s <SIGNATURE>`

:::info
This flag is required!
:::

Specifies the signature of the message.

#### `--message <MESSAGE>`

#### `-m <MESSAGE>`

:::info
This flag is required!
:::

Specifies the message that was signed.

#### `--raw`

#### `-r`

Parses the message as bytes instead of Aleo literals.

## `leo account decrypt`

[Back to Top](#subcommands)

Use this command to decrypt a record ciphertext using your Aleo private key or view key.

To decrypt a record ciphertext using your private key, run the following command

```bash
leo account decrypt --ciphertext <CIPHERTEXT> -k <KEY>
```

where `<CIPHERTEXT>` is the ciphertext of a record, and `<KEY>` is private key of the record's owner.

Optionally, you can specify a path to a file containing the key rather than the key itself:

```bash
leo account decrypt --ciphertext <CIPHERTEXT> -f <PATH_TO_KEYFILE>
```

If you do not specify either the key or key file, the CLI will attempt to use the `PRIVATE_KEY` and `VIEW_KEY` environment variables.

If the private key does not correspond to the owner of the record, the decryption will fail.

### Flags

#### `-c <CIPHERTEXT>`

:::info
This flag is required!
:::
Specifies the record ciphertext to decrypt.

#### `-k <KEY>`

Specifies the private key or view key to use for decryption. This will raise an error if you also pass the `-f` flag.

#### `-f <KEY_FILE>`

Specifies the path to a file containing the private key or view key. This will raise an error if you also pass the `-k` flag.

#### `--network <NETWORK>`

Specifies the network to deploy to. Overrides any `NETWORK` environment variable set manually or in a `.env` file. Valid network names are `testnet`, `mainnet`, and `canary`.
