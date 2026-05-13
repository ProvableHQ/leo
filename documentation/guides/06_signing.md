---
id: sign
title: Signing and Verifying
sidebar_label: Signing and Verifying
---

[general tags]: # "guides, sign, verify, signature, private_key, address"

In addition to creating accounts, `leo account` can be used to sign data and verify signatures. This can be useful for a particular class of applications that rely on signed data as input.

## Signing

The `leo account sign` command enables developers and users to create cryptographic signatures using an Aleo private key. These signatures can be verified within leo using the [`signature::verify`](../language/operators/cryptographic_operators.md#signatureverify) function or with the `leo account verify` command.

To generate a signature for Leo and Aleo values, run the following:

```bash
# replace `5field` with any aleo value
leo account sign --private-key {$PRIVATE_KEY} -m 5field

# Output:
sign1...
```

To generate a signature for any plaintext, use the `--raw` flag:

```bash
# replace "Hello, Aleo" with any plaintext message
leo account sign --private-key {$PRIVATE_KEY} -raw -m "Hello, Aleo"

# Output:
sign1...
```

There are a few alternatives to using the `--private-key` flag:

- `--private-key-file <path/to/file>` - read a private key from a text file
- no flags - read a private key from environment, or `.env`

## Verifying

To complement with the [`leo account sign`](#signing) command, the `leo account verify` command verifies the signatures of Aleo values and plaintext messages.

To verify signed aleo values, run:

```bash
# replace `5field` with the message and `sign1signaturehere` with the signature
leo account verify -a {$ADDRESS} -m 5field -s sign1signaturehere

# Output:
✅ The signature is valid

# Error Output:
Error [ECLI0377002]: cli error: ❌ The signature is invalid
```

To verify signatures of signed plaintext values, run:

```bash
# replace "Hello, Aleo" with the message and `sign1signaturehere` with the signature
leo account verify -a {$ADDRESS} --raw -m "Hello, Aleo" -s sign1signaturehere

# Output:
✅ The signature is valid

# Error Output:
Error [ECLI0377002]: cli error: ❌ The signature is invalid
```
